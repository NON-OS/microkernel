// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the USB mass-storage driver's untrusted-input
//! parsers.

#[path = "../../capsule_driver_usb_msc/src/bot/mod.rs"]
pub mod bot;
#[path = "../../capsule_driver_usb_msc/src/descriptors/mod.rs"]
pub mod descriptors;
#[path = "../../capsule_driver_usb_msc/src/disk/mod.rs"]
pub mod disk;
#[path = "../../capsule_driver_usb_msc/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_usb_msc/src/scsi/mod.rs"]
pub mod scsi;
#[path = "../../capsule_driver_usb_msc/src/span/mod.rs"]
pub mod span;
#[path = "../../capsule_driver_usb_msc/src/state/mod.rs"]
pub mod state;
/// The device behind driver.xhci0, scripted, where `disk` calls it.
pub mod xhci;

#[cfg(test)]
mod codec_tests;
#[cfg(test)]
mod msc_tests;
#[cfg(test)]
mod request_refusal_tests;
#[cfg(test)]
mod span_tests;
#[cfg(test)]
mod transport_tests;

#[cfg(kani)]
mod kani_proofs;
