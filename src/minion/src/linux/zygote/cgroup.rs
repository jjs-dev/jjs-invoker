use crate::linux::{
    jail_common::{get_path_for_cgroup_legacy_subsystem, get_path_for_cgroup_unified, JailOptions},
    util::{err_exit, Handle, Pid},
};
use std::{
    fs,
    os::unix::io::IntoRawFd,
    sync::atomic::{AtomicU8, Ordering},
};
#[derive(Clone)]
enum GroupHandles {
    // For cgroups V1, we store handles of `tasks` file in each hierarchy.
    V1(Vec<Handle>),
    // For cgroups V2, we store handle of `cgroup.procs` file in cgroup dir.
    V2(Handle),
}

#[derive(Clone)]
pub(in crate::linux) struct Group {
    handles: GroupHandles,
    id: String,
}

impl Group {
    pub(super) fn join_self(&self) {
        let mut slice_iter;
        let mut once_iter;
        let it: &mut dyn std::iter::Iterator<Item = Handle> = match &self.handles {
            GroupHandles::V1(handles) => {
                slice_iter = handles.iter().map(|x| *x);
                &mut slice_iter
            }
            GroupHandles::V2(handle) => {
                once_iter = std::iter::once(*handle);
                &mut once_iter
            }
        };
        let my_pid = std::process::id();
        let my_pid = format!("{}", my_pid);
        for h in it {
            nix::unistd::write(h, my_pid.as_bytes()).expect("Couldn't join cgroup");
        }
    }
}

enum CgroupVersion {
    V1,
    V2,
}

const CGROUP_VERSION_1: u8 = 1;
const CGROUP_VERSION_2: u8 = 2;

fn do_detect_cgroup_version() -> u8 {
    let stat =
        nix::sys::statfs::statfs("/sys/fs/cgroup").expect("/sys/fs/cgroup is not root of cgroupfs");
    let ty = stat.filesystem_type();
    // TODO: this is hack. Remove as soon as possible. See https://github.com/nix-rust/nix/pull/1187 and https://github.com/rust-lang/libc/pull/1660/
    let ty: libc::c_long = unsafe { std::mem::transmute(ty) };
    // man 2 statfs
    match ty {
        0x27e0eb => CGROUP_VERSION_1,
        0x63677270 => CGROUP_VERSION_2,
        other_fs_magic => panic!("unknown FS magic: {:x}", other_fs_magic),
    }
}

fn detect_cgroup_version() -> CgroupVersion {
    static CACHE: AtomicU8 = AtomicU8::new(0);
    if CACHE.load(Ordering::Relaxed) == 0 {
        let version = do_detect_cgroup_version();
        CACHE.store(version, Ordering::Relaxed);
    }
    match CACHE.load(Ordering::Relaxed) {
        CGROUP_VERSION_1 => CgroupVersion::V1,
        CGROUP_VERSION_2 => CgroupVersion::V2,
        val => unreachable!("unexpected value in cgroup version cache: {}", val),
    }
}

unsafe fn setup_chroups_legacy(jail_options: &JailOptions) -> Vec<Handle> {
    let jail_id = &jail_options.jail_id;
    // configure cpuacct subsystem
    let cpuacct_cgroup_path = get_path_for_cgroup_legacy_subsystem("cpuacct", &jail_id);
    fs::create_dir_all(&cpuacct_cgroup_path).expect("failed to create cpuacct cgroup");

    // configure pids subsystem
    let pids_cgroup_path = get_path_for_cgroup_legacy_subsystem("pids", &jail_id);
    fs::create_dir_all(&pids_cgroup_path).expect("failed to create pids cgroup");

    fs::write(
        pids_cgroup_path.join("pids.max"),
        format!("{}", jail_options.max_alive_process_count),
    )
    .expect("failed to enable pids limit");

    //configure memory subsystem
    let mem_cgroup_path = get_path_for_cgroup_legacy_subsystem("memory", &jail_id);

    fs::create_dir_all(&mem_cgroup_path).expect("failed to create memory cgroup");
    fs::write(mem_cgroup_path.join("memory.swappiness"), "0").expect("failed to disallow swapping");

    fs::write(
        mem_cgroup_path.join("memory.limit_in_bytes"),
        format!("{}", jail_options.memory_limit),
    )
    .expect("failed to enable memory limiy");

    let my_pid: Pid = libc::getpid();
    if my_pid == -1 {
        err_exit("getpid");
    }

    // we return handles to tasksfiles for main cgroups
    // so, though zygote itself and children are in chroot, and cannot access cgroupfs, they will be able to add themselves to cgroups
    ["cpuacct", "memory", "pids"]
        .iter()
        .map(|subsys_name| {
            let p = get_path_for_cgroup_legacy_subsystem(subsys_name, &jail_id);
            let p = p.join("tasks");
            let h = fs::OpenOptions::new()
                .write(true)
                .open(&p)
                .unwrap_or_else(|err| panic!("Couldn't open tasks file {}: {}", p.display(), err))
                .into_raw_fd();
            libc::dup(h)
        })
        .collect::<Vec<_>>()
}

unsafe fn setup_cgroups_v2(jail_options: &JailOptions) -> Handle {
    let jail_id = &jail_options.jail_id;
    let cgroup_path = get_path_for_cgroup_unified(jail_id);
    fs::create_dir_all(&cgroup_path).expect("failed to create cgroup");

    fs::write(cgroup_path.parent().unwrap().join("cgroup.subtree_control"), "+pids +cpu +memory").expect("failed to enable controllers");

    fs::write(
        cgroup_path.join("pids.max"),
        format!("{}", jail_options.max_alive_process_count),
    )
    .expect("failed to set pids.max limit");

    fs::write(
        cgroup_path.join("memory.max"),
        format!("{}", jail_options.memory_limit),
    )
    .expect("failed to set memory limit");

    let tasks_file_path = cgroup_path.join("cgroup.procs");
    let h = fs::OpenOptions::new()
        .write(true)
        .open(&tasks_file_path)
        .unwrap_or_else(|err| {
            panic!(
                "Failed to open tasks file {}: {}",
                tasks_file_path.display(),
                err
            )
        });
    libc::dup(h.into_raw_fd())
}

pub(super) unsafe fn setup_cgroups(jail_options: &JailOptions) -> Group {
    let handles = match detect_cgroup_version() {
        CgroupVersion::V1 => GroupHandles::V1(setup_chroups_legacy(jail_options)),
        CgroupVersion::V2 => GroupHandles::V2(setup_cgroups_v2(jail_options)),
    };
    Group {
        handles,
        id: jail_options.jail_id.clone(),
    }
}
