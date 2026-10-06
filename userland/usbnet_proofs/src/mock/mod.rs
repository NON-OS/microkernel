// NONOS Operating System (AGPL-3.0-or-later)
//! A scripted USB device behind `Bus`.

mod bus;
mod device;
mod script;

pub use bus::MockBus;
pub use device::descriptors;
pub use script::{Call, Responder, Script};
