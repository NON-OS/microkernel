// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's server as it ships, its replies recorded instead of sent:
//! whom it admits, where each request goes, and its two bounded halves.

#[path = "../../../capsule_driver_usb_msc/src/server/admits.rs"]
pub mod admits;
#[path = "../../../capsule_driver_usb_msc/src/server/dispatch.rs"]
mod dispatch;
#[path = "../../../capsule_driver_usb_msc/src/server/handlers/mod.rs"]
pub mod handlers;
pub mod respond;
#[path = "../../../capsule_driver_usb_msc/src/server/serve.rs"]
pub mod serve;
