// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the USB microphone driver. Its modules are
//! mounted at the paths it names them by (`crate::uac`, `crate::wire`),
//! and the USB audio proofs' device models beside them.

extern crate alloc;

/// driver.uvc0's isochronous batch reader, which the microphone mounts.
#[path = "../../capsule_driver_uvc/src/batch.rs"]
pub mod batch;

/// USB Audio 1.0 descriptors and devices (usb_audio_proofs').
#[path = "../../usb_audio_proofs/src/devices.rs"]
pub mod devices;
#[path = "../../capsule_driver_usb_mic/src/frame.rs"]
pub mod frame;
#[path = "../../capsule_driver_usb_mic/src/gate.rs"]
pub mod gate;
#[path = "../../capsule_driver_usb_mic/src/ring.rs"]
pub mod ring;
#[path = "../../capsule_driver_usb_mic/src/uac/mod.rs"]
pub mod uac;
/// USB Audio 2.0 descriptors (usb_audio_proofs').
#[path = "../../usb_audio_proofs/src/uac2.rs"]
pub mod uac2;
#[path = "../../capsule_driver_usb_mic/src/wire.rs"]
pub mod wire;

/// A seeded sweep over the streaming alternates a function offers.
#[cfg(test)]
mod alt_sweep;
#[cfg(test)]
mod alt_sweep_tests;
/// A seeded sweep over captured sound.
#[cfg(test)]
mod capture_sweep;
#[cfg(test)]
mod capture_sweep_tests;
#[cfg(test)]
mod capture_tests;
#[cfg(test)]
mod gate_tests;
#[cfg(test)]
mod ring_tests;
