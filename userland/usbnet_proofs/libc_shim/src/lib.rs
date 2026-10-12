// NONOS Operating System (AGPL-3.0-or-later)
//! The calls the USB network core makes into nonos_libc, answered on the
//! host. Standing in, not stubbing out: an IPC call reaches whatever plays
//! driver.xhci0 in the proof, entropy comes from a generator a proof can
//! switch off, each log line is kept, and the uptime clock moves only when
//! the proof advances it, so a test says what a slow device costs.
//!
//! Every piece of state here is per thread, as each test runs on its own;
//! `serial` keeps proofs that reach the core's own statics apart.

mod debug;
mod entropy;
mod inbox;
mod ipc;
mod serial;
mod time;

pub use debug::{lines, mk_debug};
pub use entropy::{crypto_random, set_entropy};
pub use inbox::{mk_ipc_recv_from, mk_ipc_reply};
pub use ipc::{calls, mk_ipc_call_timeout, mk_service_lookup, play, Asked};
pub use serial::serial;
pub use time::{advance, mk_uptime_ms, Deadline};
