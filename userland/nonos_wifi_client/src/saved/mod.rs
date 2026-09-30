/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The networks a person chose to remember.
//!
//! `list` is the plaintext slots, `file` seals and opens them, `key` asks the
//! TPM for the sealing key, and `store` reads and writes the record through
//! vfs, refusing to write one on a boot that keeps nothing.

mod error;
mod file;
mod key;
mod list;
mod list_codec;
mod store;
mod write;

pub use error::SavedError;
pub use list::{SavedList, PASS_MAX, SLOTS};
pub use store::{forget, load, remember};
pub use write::{forget_all, keeps_state, sealing_ready};
