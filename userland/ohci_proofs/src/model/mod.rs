// NONOS Operating System (AGPL-3.0-or-later)
//! The model: an OHCI controller's registers and root hub, a frame runner
//! that walks the periodic, control and bulk lists the driver built as the
//! controller would, and the devices of uhci_proofs (mounted), which
//! answer each packet and check its data toggle. Written from the OpenHCI
//! specification and USB 2.0 as recalled.

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
mod frame;
mod handles;
#[path = "../../../uhci_proofs/src/model/hub.rs"]
pub mod hub;
#[path = "../../../uhci_proofs/src/model/make.rs"]
mod make;
#[path = "../../../uhci_proofs/src/model/pid.rs"]
pub mod pid;
mod ports;
mod regs;
mod retire;
mod steps;
mod td;
#[path = "../../../uhci_proofs/src/model/wire.rs"]
mod wire;
mod world;

pub use device::{Device, Reply};
pub use handles::{rig, Cx, ModelClock, ModelMem, ModelRegs, Shared};
pub use steps::{addressed, bulk_ready};
pub use wire::{call, call_host, Answer};
pub use world::{World, BUS, PORTS};
