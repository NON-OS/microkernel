// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the USB HID driver over untrusted device data:
//! the configuration descriptor walk and every interrupt-IN report decode.

extern crate alloc;

pub mod descriptors;
pub mod enumerate;
/// The bounded search the driver finds its host controller with.
#[path = "../../capsule_driver_usb_hid/src/orchestrator/find_within.rs"]
pub mod find_within;
pub mod hid;
pub mod hub;
pub mod poll;
pub mod protocol;
/// The driver's whole request wire: the header decode every frame passes
/// before dispatch, and the encoders a refusal is answered with. Its limits
/// file is also `protocol`'s, so it is loaded twice on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../../capsule_driver_usb_hid/src/protocol/mod.rs"]
pub mod usb_hid_wire;
/// The controller driver's request limits, which the driver's reads must
/// stay inside.
#[path = "../../capsule_driver_xhci/src/protocol/limits.rs"]
pub mod xhci_limits;

#[cfg(test)]
mod boundary_tests;
#[cfg(test)]
mod composite_tests;
#[cfg(test)]
mod config_blob;
#[cfg(test)]
mod descriptor_walk_tests;
#[cfg(test)]
mod find_within_tests;
#[cfg(test)]
mod fuzz_tests;
#[cfg(test)]
mod hub_descriptor_tests;
#[cfg(test)]
mod hub_port_tests;
#[cfg(test)]
mod hub_route_tests;
#[cfg(test)]
mod keyboard_tests;
#[cfg(test)]
mod mouse_tests;
#[cfg(test)]
mod read_len_tests;
#[cfg(test)]
mod request_refusal_tests;
#[cfg(test)]
mod rng;
#[cfg(test)]
mod tablet_tests;
#[cfg(test)]
mod unplug_tests;
#[cfg(test)]
mod usb_tests;
