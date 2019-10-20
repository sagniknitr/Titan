pub mod error {
    use failure::Fail;

    #[derive(Debug)]
    pub struct titanError {
        pub code: titanStatusCode,
        pub message: &'static str,
    }

    #[derive(Debug, Fail)]
    pub enum titanStatusCode {
        #[fail(display = "success")]
        Ok = 0,
        #[fail(display = "unspecified error")]
        Error = 1,
        #[fail(display = "timeout occurred")]
        Timeout = 2,
        #[fail(display = "failed to allocate memory")]
        BadAlloc = 4,
        #[fail(display = "invalid argument")]
        InvalidArgument = 8,
        #[fail(display = "context already initialized")]
        AlreadyInit = 16,
        #[fail(display = "context not yet initialized")]
        NotInit = 32,
        #[fail(display = "context not yet initialized")]
        MismatchedTile = 64,
        #[fail(display = "context not yet initialized")]
       
    }

    impl From<i32> for titanStatusCode {
        fn from(error: i32) -> Self {
            match error {
                0   => titanStatusCode::Ok,
                1   => titanStatusCode::Error,
                2   => titanStatusCode::Timeout,
                4   => titanStatusCode::BadAlloc,
                8   => titanStatusCode::InvalidArgument,
                16  => titanStatusCode::AlreadyInit,
                32  => titanStatusCode::NotInit,
                64  => titanStatusCode::MismatchedTile,
                _ => unimplemented!(),
            }
        }
    }
}

pub mod traits {
    use downcast::{
        downcast, downcast_methods, downcast_methods_core, downcast_methods_std, impl_downcast, Any,
    };
    use libc::uintptr_t;

    pub trait Message: Any {
        fn get_native_message(&self) -> uintptr_t;
        fn destroy_native_message(&self, message_handle: uintptr_t) -> ();
        fn read_handle(&mut self, message_handle: uintptr_t) -> ();
    }

    downcast!(Message);

    pub trait MessageDefinition<T>: Message {
        fn get_type_support() -> uintptr_t;
        fn static_get_native_message(message: &T) -> uintptr_t;
        fn static_destroy_native_message(message_handle: uintptr_t) -> ();
    }
}
