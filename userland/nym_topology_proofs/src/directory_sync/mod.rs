// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's directory code that turns the validator's described nodes
//! into exits. The TLS fetch is the one piece that needs a network; its
//! stand-in here always fails, so a test can only pass through the parser.

pub mod api;
pub mod https;
pub mod live;
#[path = "../../../capsule_net_nym/src/directory_sync/requesters.rs"]
pub mod requesters;
