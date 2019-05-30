#!/bin/bash

set -e

SELF="$0"

if [ "${SELF:0:1}" != / ]
then SELF="$(pwd)/$SELF"
fi

if [ "$$" != 1 ]
then exec linux.uml root=/dev/root rw rootflags=/ rootfstype=hostfs init="$SELF"
fi

cd "$(dirname "$0")"

dd if=/dev/null of=hdd.img bs=1048576 seek=512
mke2fs hdd.img
insmod /usr/lib/uml/modules/$(uname -r)/kernel/drivers/block/loop.ko
mount -t proc proc /proc
cat /proc/modules
losetup /dev/loop0 hdd.img
mount -t ext4 /dev/loop0 /mnt
export ORIG_CWD="$(pwd)"
cd /mnt
ln -s /proc/self/fd /dev/fd
tar -xvf <(bash -c 'cd "$ORIG_CWD/../sysroot"; tar -cvf - .')
cd /
umount /mnt
sync
poweroff -f
