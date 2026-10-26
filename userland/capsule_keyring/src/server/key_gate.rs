// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The key gate, asked of the registry on every request: only the names in
//! `protocol::callers` create, import or sign.

use crate::protocol::{may_ask, CREATORS};
use crate::store::{MAX_KEYS, MAX_KEYS_PER_OWNER};

// Only the creators can come to own keys, each at most its share, so the
// wallet never meets a store another owner filled. A name added beyond the
// store's room fails the build.
const _: () = assert!(CREATORS.len() * MAX_KEYS_PER_OWNER <= MAX_KEYS);

/// Whether `sender_pid` may ask `op`, by the registry's word now.
pub fn allowed(op: u16, sender_pid: u32) -> bool {
    may_ask(op, sender_pid, super::registry::owner_of)
}
