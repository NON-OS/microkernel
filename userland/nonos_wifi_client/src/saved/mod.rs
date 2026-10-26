/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The networks a person chose to remember.
//!
//! `list` is the plaintext slots, `file` seals and opens them, `key` asks the
//! TPM for the sealing key, and `store` reads and writes the record through
//! vfs, refusing to write one on a boot that keeps nothing.

mod closed;
mod dropped;
mod error;
mod file;
mod keep;
mod key;
mod list;
mod list_codec;
mod list_put;
mod pass;
mod pin;
mod store;
mod write;

pub use closed::{SavedFlags, SAVED_UNREADABLE};
pub use dropped::Dropped;
pub use error::SavedError;
pub use keep::{forget, pin_wpa3, saved_flags};
pub use list::{SavedList, PASS_MAX, SLOTS};
pub use store::{load, remember, remember_with};
pub use write::{forget_all, keeps_state, sealing_ready};
