// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's directory code that turns the validator's described nodes
//! into exits. The TLS fetch is the one piece that needs a network; its
//! stand-in here always fails, so a test can only pass through the parser.

pub mod api;
#[path = "../../../capsule_net_nym/src/directory_sync/described.rs"]
pub mod described;
#[path = "../../../capsule_net_nym/src/directory_sync/exit_address.rs"]
pub mod exit_address;
pub mod https;
pub mod live;
#[path = "../../../capsule_net_nym/src/directory_sync/requesters.rs"]
pub mod requesters;
