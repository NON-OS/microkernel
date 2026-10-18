// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the USB mass-storage driver's untrusted-input
//! parsers.

// The included driver files take `Vec` from `alloc`, as a no_std crate does.
extern crate alloc;
// The driver's xhci/call.rs calls driver.xhci0 through nonos_libc; here that
// is this crate, whose root has the scripted controller's end of the call.
extern crate self as nonos_libc;
pub(crate) use xhci::mk_ipc_call_timeout;

#[path = "../../capsule_driver_usb_msc/src/bot/mod.rs"]
pub mod bot;
/// The driver's clock, kept by the tests.
pub mod clock;
#[path = "../../capsule_driver_usb_msc/src/descriptors/mod.rs"]
pub mod descriptors;
#[path = "../../capsule_driver_usb_msc/src/disk/mod.rs"]
pub mod disk;
#[path = "../../capsule_driver_usb_msc/src/protocol/mod.rs"]
pub mod protocol;
#[path = "../../capsule_driver_usb_msc/src/scan/mod.rs"]
pub mod scan;
#[path = "../../capsule_driver_usb_msc/src/scsi/mod.rs"]
pub mod scsi;
#[path = "../../capsule_driver_usb_msc/src/span/mod.rs"]
pub mod span;
#[path = "../../capsule_driver_usb_msc/src/state/mod.rs"]
pub mod state;
/// The device behind driver.xhci0, scripted, where `disk` calls it.
pub mod xhci;

#[cfg(test)]
mod answers;
#[cfg(test)]
mod change_tests;
#[cfg(test)]
mod fixture;
#[cfg(test)]
mod kernel;
#[cfg(test)]
mod server;
#[cfg(test)]
mod serving;
#[cfg(test)]
mod sticks;
#[cfg(test)]
mod tests;

#[cfg(kani)]
mod kani_proofs;
