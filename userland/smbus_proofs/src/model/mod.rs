// NONOS Operating System (AGPL-3.0-or-later)
//! A model of an i801 host: its registers as the datasheet has them, the
//! INUSE semaphore and write-one-to-clear status, the 32-byte block buffer
//! and its index, and SMBus devices that answer at their address.

mod device;
mod host;
mod ports;
mod run;

pub use device::Device;
pub use host::Model;
