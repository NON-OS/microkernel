// NONOS Operating System (AGPL-3.0-or-later)
//! net.core's own reading of a batch reply, mounted unmodified, so a batch
//! test proves what net.core takes and not what a copy of it would.

#[path = "../../../capsule_net_core/src/device/batch_frames.rs"]
mod frames;
#[path = "../../../capsule_net_core/src/device/batch_served.rs"]
mod served;

pub use frames::batch_frames;
pub use served::served;
