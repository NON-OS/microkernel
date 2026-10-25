// NONOS Operating System (AGPL-3.0-or-later)

mod uptime;

#[path = "../../../libc/src/time/deadline.rs"]
mod deadline;

pub use deadline::Deadline;
pub use uptime::{mk_uptime_ms, set_uptime};
