// NONOS Operating System (AGPL-3.0-or-later)
//! The model: a UHCI controller's registers and root ports, a frame runner
//! that walks the schedule the driver built as the controller would, and
//! full- and low-speed devices that answer each packet and check its data
//! toggle. Written from the UHCI Design Guide and USB 2.0 as recalled.

mod device;
mod device_io;
mod ep0;
mod frame;
mod handles;
mod make;
mod ports;
mod regs;
mod steps;
mod transact;
mod wire;
mod world;

pub use device::{Device, Reply};
pub use handles::{rig, Cx, ModelClock, ModelIo, ModelMem, Shared};
pub use steps::{addressed, bulk_ready};
pub use wire::{call, Answer};
pub use world::{World, BUS};
