// NONOS Operating System (AGPL-3.0-or-later)
//! The model: an EHCI controller's registers and root ports (each with a
//! device's speed, so full- and low-speed devices end with the companion),
//! a frame runner that walks the periodic list and the async ring the
//! driver built as the controller would, running qTDs through each QH's
//! overlay, the async advance doorbell, and the devices of uhci_proofs
//! (mounted). Written from the EHCI specification and USB 2.0 as recalled.

#[path = "../../../uhci_proofs/src/model/bus.rs"]
pub mod bus;
#[path = "../../../uhci_proofs/src/model/device.rs"]
mod device;
#[path = "../../../uhci_proofs/src/model/device_io.rs"]
mod device_io;
#[path = "../../../uhci_proofs/src/model/ep0.rs"]
mod ep0;
#[path = "../../../uhci_proofs/src/model/ep0_setup.rs"]
mod ep0_setup;
mod exec;
mod frame;
mod handles;
#[path = "../../../uhci_proofs/src/model/hub.rs"]
pub mod hub;
#[path = "../../../uhci_proofs/src/model/make.rs"]
mod make;
mod packets;
#[path = "../../../uhci_proofs/src/model/pid.rs"]
pub mod pid;
mod ports;
mod reach;
mod regs;
mod retire;
mod steps;
mod wire;
mod world;

pub use device::{Device, Reply};
pub use handles::{rig, Cx, ModelClock, ModelMem, ModelRegs, Shared};
pub use steps::{addressed, bulk_ready, high};
pub use wire::{call, Answer};
pub use world::{Speed, World, BUS, CAPLEN, PORTS};
