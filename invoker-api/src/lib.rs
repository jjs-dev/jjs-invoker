use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Clone, Debug, Display, PartialEq, Eq, Serialize, Deserialize)]
pub enum StatusKind {
    Queue,
    /// WA, TLE, rejected by teacher, etc
    Rejected,
    /// e.g. Coding Style Violation
    CompilationError,
    // Evaluated,
    Accepted,
    NotSet,
    InternalError,
}

pub mod status_codes {
    macro_rules! declare_code {
        ($code: ident) => {
            pub const $code: &str = stringify!($code);
        };

        ($code: ident, $($codes: ident),+) => {
             declare_code!($code);
             declare_code!($($codes),+);
        };
    }

    // build-related status codes
    declare_code!(
        TOOLCHAIN_SEARCH_ERROR,
        BUILT,
        COMPILATION_TIMED_OUT,
        COMPILER_FAILED
    );

    // per-test status codes
    declare_code!(
        TIME_LIMIT_EXCEEDED,
        RUNTIME_ERROR,
        TEST_PASSED,
        JUDGE_FAULT,
        WRONG_ANSWER,
        PRESENTATION_ERROR
    );

    // aggregated status codes
    declare_code!(ACCEPTED, BUILD_ERROR);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub kind: StatusKind,
    pub code: String,
}