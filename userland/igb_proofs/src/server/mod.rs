// NONOS Operating System (AGPL-3.0-or-later)
//! The server: the loop, the dispatch, the handlers and the one reply path,
//! against the IPC calls the shim scripts.

#[path = "../../../capsule_driver_igb/src/server/dispatch.rs"]
pub mod dispatch;
#[path = "../../../capsule_driver_igb/src/server/error.rs"]
pub mod error;
#[path = "../../../capsule_driver_igb/src/server/handlers/mod.rs"]
pub mod handlers;
#[path = "../../../capsule_driver_igb/src/server/runner.rs"]
pub mod runner;
