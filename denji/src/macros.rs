#[macro_export]
macro_rules! args {
    ($ ( $arg:expr ),+ $(,)?) => {
        {
            let args: Vec<std::ffi::OsString> = Vec::from([$(std::ffi::OsString::from($arg), )+]);

            args
        }
    }
}
