macro_rules! _trace {
    ($color:literal, $tag:literal, $args:expr) => {
        eprintln!(
            "\x1b[1m{}{:>count$}\x1b[0m │ {}",
            $color,
            $tag,
            $args,
            count = 7
        )
    };
}

macro_rules! _log {
    ($($args:tt)*) => {
        $crate::macros::trace!(
            "\x1b[38;5;71m",
            "log",
            format_args!($($args)*)
        );
    };
}

macro_rules! _info {
    ($($args:tt)*) => {
        $crate::macros::trace!(
            "\x1b[36m",
            "info",
            format_args!($($args)*)
        );
    };
}

macro_rules! _warn {
    ($($args:tt)*) => {
        $crate::macros::trace!(
            "\x1b[33m",
            "warning",
            format_args!($($args)*)
        );
    };
}

macro_rules! _error {
    ($($args:tt)*) => {
        $crate::macros::trace!(
            "\x1b[31m",
            "error",
            format_args!($($args)*)
        );
    };
}

pub(crate) use _error as error;
pub(crate) use _info as info;
pub(crate) use _log as log;
pub(crate) use _trace as trace;
pub(crate) use _warn as warn;
