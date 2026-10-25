// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's BOT command and what it calls, mounted on their own: the
//! disk module keeps them private behind the transport, and they refer to
//! one another only through `super`, so the sweep reaches them as built.

#![allow(dead_code)]

#[path = "../../../capsule_driver_usb_msc/src/disk/bot.rs"]
pub mod bot;
#[path = "../../../capsule_driver_usb_msc/src/disk/csw.rs"]
mod csw;
#[path = "../../../capsule_driver_usb_msc/src/disk/data.rs"]
mod data;
#[path = "../../../capsule_driver_usb_msc/src/disk/data_phase.rs"]
mod data_phase;
#[path = "../../../capsule_driver_usb_msc/src/disk/recover.rs"]
mod recover;
#[path = "../../../capsule_driver_usb_msc/src/disk/types.rs"]
pub mod types;
