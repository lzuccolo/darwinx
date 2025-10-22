#[macro_export]
macro_rules! register_indicator {
    ($metadata_fn:expr) => {
        // Constructor estático que se ejecuta antes de main()
        #[::ctor::ctor]
        fn __register_indicator() {
            $crate::registry::register($metadata_fn());
        }
    };
}