// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the USB audio driver. The shipped modules are
//! mounted at the paths the driver names (`crate::uac`, `crate::wire`).

extern crate alloc;

/// The pure half of `xhci`: reading a port's speed.
#[path = "../../capsule_driver_usb_audio/src/xhci/portsc.rs"]
pub mod portsc;
/// The answers and the queue; the loop around them calls the kernel.
pub mod serve;
#[path = "../../capsule_driver_usb_audio/src/uac/mod.rs"]
pub mod uac;
#[path = "../../capsule_driver_usb_audio/src/wire/mod.rs"]
pub mod wire;

/// Descriptors of the devices the proofs bind.
pub mod devices;
/// What the other side of each wire reads: driver.hda0's numbering and the
/// audio server's sink client.
pub mod peers;
/// USB Audio 2.0 descriptors and a 2.0 DAC.
pub mod uac2;

#[cfg(test)]
mod answer_tests;
#[cfg(test)]
mod ask;
#[cfg(test)]
mod hostile_tests;
#[cfg(test)]
mod loss_tests;
#[cfg(test)]
mod peer_tests;
#[cfg(test)]
mod playback_tests;
#[cfg(test)]
mod queue_tests;
#[cfg(test)]
mod refusal_tests;
#[cfg(test)]
mod request_tests;
#[cfg(test)]
mod terminal_tests;
#[cfg(test)]
mod timing_tests;
#[cfg(test)]
mod uac2_tests;
#[cfg(test)]
mod unplug_tests;
#[cfg(test)]
mod walk_tests;
