#[macro_export]
macro_rules! strict {
    ($e:expr, $($toks:tt)+) => {
        {
            let x: bool = $e;
            if cfg!(feature = "strict_checks") && $e == false {
                $($toks)+
            }
        }
    };
}
