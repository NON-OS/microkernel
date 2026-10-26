// NONOS Operating System (AGPL-3.0-or-later)
//! A plain `linux` run asking to start again with the network: once, and
//! handed its input again from the start.

#[path = "../../libc/src/linux_ask_net.rs"]
pub mod linux_ask_net;
#[path = "../../capsule_terminal/src/jobs/net_ask.rs"]
pub mod net_ask;
#[path = "../../capsule_terminal/src/jobs/stdin_kept.rs"]
pub mod stdin_kept;

mod net_ask_tests;
