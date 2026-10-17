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

/// The driver's BOT command, mounted where the sweep reaches it.
#[cfg(test)]
mod bot_parts;
/// A seeded sweep over BOT commands.
#[cfg(test)]
mod bot_sweep;
/// A seeded sweep over the SCSI answers a disk is sized and flushed by.
#[cfg(test)]
mod scsi_sweep;

#[cfg(test)]
mod bot_sweep_tests;
#[cfg(test)]
mod codec_tests;
#[cfg(test)]
mod csw_shape_tests;
#[cfg(test)]
mod msc_tests;
#[cfg(test)]
mod request_refusal_tests;
#[cfg(test)]
mod scsi_sweep_tests;
#[cfg(test)]
mod sense_tests;
#[cfg(test)]
mod span_tests;
#[cfg(test)]
mod transport_tests;
#[cfg(test)]
mod uas_tag_tests;
#[cfg(test)]
mod uas_tests;

#[cfg(kani)]
mod kani_proofs;
