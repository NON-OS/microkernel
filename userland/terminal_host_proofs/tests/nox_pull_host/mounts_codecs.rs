// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors

//! The inflate and base64 crates, linked as the capsule links them and named
//! by the module paths the codec tests use.

pub mod inflate {
    pub mod gzip {
        pub use nonos_inflate::gunzip;
    }
}

pub mod base64 {
    pub mod encode {
        pub use nonos_base64::encode;
    }
}
