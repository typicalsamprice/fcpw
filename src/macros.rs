#[macro_export]
macro_rules! strict_eq {
    ($left:expr, $right:expr) => {
        if cfg!(feature = "strict_checks") {
            assert_eq!($left, $right);
        }
    };
    ($left:expr, $right:expr, $($toks:tt)+) => {
        if cfg!(feature = "strict_checks") && ($left) != ($right) {
            $($toks)+
        }
    };
}
#[macro_export]
macro_rules! strict_ne {
    ($left:expr, $right:expr) => {
        if cfg!(feature = "strict_checks") {
            assert_ne!($left, $right);
        }
    };

    ($left:expr, $right:expr, $($toks:tt)+) => {
        if cfg!(feature = "strict_checks") && ($left) == ($right) {
            $($toks)+
        }
    };
}
#[macro_export]
macro_rules! strict_cond {
    ($e:expr) => {
        if cfg!(feature = "strict_checks") {
            assert!($e);
        }
    };
    ($e:expr, $($toks:tt)+) => {
        if cfg!(feature = "strict_checks") && ($e) == false {
            $($toks)+
        }
    };
}
#[macro_export]
macro_rules! strict_not {
    ($e:expr) => {
        if cfg!(feature = "strict_checks") {
            assert!(!($e));
        }
    };
    ($e:expr, $($toks:tt)+) => {
        if cfg!(feature = "strict_checks") && ($e) == true {
            $($toks)+
        }
    };
}
