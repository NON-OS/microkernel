// NONOS Operating System (AGPL-3.0-or-later)
//! The serve modules that touch no kernel call.

#[path = "../../../capsule_driver_usb_audio/src/serve/answer.rs"]
pub mod answer;
#[path = "../../../capsule_driver_usb_audio/src/serve/loss.rs"]
pub mod loss;
#[path = "../../../capsule_driver_usb_audio/src/serve/play.rs"]
pub mod play;
#[path = "../../../capsule_driver_usb_audio/src/serve/queue.rs"]
pub mod queue;
#[path = "../../../capsule_driver_usb_audio/src/serve/reply.rs"]
pub mod reply;
