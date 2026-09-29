// NONOS Operating System (AGPL-3.0-or-later)

pub mod serial;

pub use serial::{write_str as serial_write_str, SerialWriter};
