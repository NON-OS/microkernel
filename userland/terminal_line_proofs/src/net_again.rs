// NONOS Operating System (AGPL-3.0-or-later)
//! A plain `linux` run asking to start again with the network: once, and
//! handed its input again from the start.

#[path = "../../capsule_terminal/src/jobs/net_ask.rs"]
pub mod net_ask;
#[path = "../../capsule_terminal/src/jobs/stdin_kept.rs"]
pub mod stdin_kept;
/* Mounted for LINUX_ASK_NET alone; the frame itself is the capsule's. */
#[allow(dead_code)]
#[path = "../../libc/src/foreign_frame.rs"]
pub mod foreign_frame;

mod net_ask_tests;
