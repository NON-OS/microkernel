// NONOS Operating System (AGPL-3.0-or-later)

//! The shipping controller files that touch only registers and DMA memory.
//! `park` and the waits are public for the paths siblings and tests use;
//! the handoff's two files are siblings so `claim` resolves through `super`.

#[path = "../../../capsule_driver_xhci/src/controller/bulk/mod.rs"]
mod bulk;
#[path = "../../../capsule_driver_xhci/src/controller/legacy_handoff/claim.rs"]
mod claim;
#[path = "../../../capsule_driver_xhci/src/controller/drain_events.rs"]
mod drain_events;
mod exports;
#[path = "../../../capsule_driver_xhci/src/controller/get_device_descriptor.rs"]
mod get_device_descriptor;
#[path = "../../../capsule_driver_xhci/src/controller/halt.rs"]
mod halt;
#[path = "../../../capsule_driver_xhci/src/controller/isoch/mod.rs"]
mod isoch;
#[path = "../../../capsule_driver_xhci/src/controller/issue_address_device.rs"]
mod issue_address_device;
#[path = "../../../capsule_driver_xhci/src/controller/issue_enable_slot.rs"]
mod issue_enable_slot;
#[path = "../../../capsule_driver_xhci/src/controller/legacy_handoff/legacy_handoff.rs"]
mod legacy_handoff;
#[path = "../../../capsule_driver_xhci/src/controller/park.rs"]
pub mod park;
#[path = "../../../capsule_driver_xhci/src/controller/poll_interrupt_in/mod.rs"]
mod poll_interrupt_in;
#[path = "../../../capsule_driver_xhci/src/controller/program_command_ring.rs"]
mod program_command_ring;
#[path = "../../../capsule_driver_xhci/src/controller/program_event_ring.rs"]
mod program_event_ring;
#[path = "../../../capsule_driver_xhci/src/controller/recover_endpoint.rs"]
mod recover_endpoint;
#[path = "../../../capsule_driver_xhci/src/controller/refuse_unsupported.rs"]
mod refuse_unsupported;
#[path = "../../../capsule_driver_xhci/src/controller/reset.rs"]
mod reset;
#[path = "../../../capsule_driver_xhci/src/controller/reset_port.rs"]
mod reset_port;
#[path = "../../../capsule_driver_xhci/src/controller/reset_toggle.rs"]
mod reset_toggle;
#[path = "../../../capsule_driver_xhci/src/controller/ring_doorbell.rs"]
mod ring_doorbell;
#[path = "../../../capsule_driver_xhci/src/controller/run_command.rs"]
mod run_command;
#[path = "../../../capsule_driver_xhci/src/controller/scratchpad.rs"]
mod scratchpad;
#[path = "../../../capsule_driver_xhci/src/controller/start.rs"]
mod start;
#[path = "../../../capsule_driver_xhci/src/controller/wait_cnr_clear.rs"]
mod wait_cnr_clear;
#[path = "../../../capsule_driver_xhci/src/controller/wait_command_completion.rs"]
pub mod wait_command_completion;
#[path = "../../../capsule_driver_xhci/src/controller/wait_hc_running.rs"]
mod wait_hc_running;
#[path = "../../../capsule_driver_xhci/src/controller/transfer_events.rs"]
pub mod transfer_events;
#[path = "../../../capsule_driver_xhci/src/controller/wait_transfer_completion.rs"]
pub mod wait_transfer_completion;

pub use exports::*;
