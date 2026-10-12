// NONOS Operating System (AGPL-3.0-or-later)
//! The command path end to end, on the model port and drive.

mod before_issue;
mod budget;
mod budget_reply;
mod buffer;
/// What a failed request answers its client.
#[path = "../../../../capsule_driver_ahci/src/server/handlers/failure.rs"]
mod failure;
mod fatal;
mod fis;
mod flush;
mod foreign;
mod identify;
mod pace;
mod prdbc;
mod replies;
mod self_reset;
mod slow_cr;
mod slow_reset;
mod slow_rig;
mod slow_window;
mod stale_err;
mod timeout;
mod unlinked;
mod wedged;
