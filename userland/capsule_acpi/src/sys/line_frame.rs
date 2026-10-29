// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A serial line as the kernel takes it: at most `max` bytes in one call,
//! ending in its own newline. The kernel passes a capsule's bytes through as
//! they are, so a line without one runs into the next. Pure, for the proofs.

use alloc::vec::Vec;

/// `line` cut to leave room for the newline within `max`, then the newline.
pub fn framed(mut line: Vec<u8>, max: usize) -> Vec<u8> {
    line.truncate(max.saturating_sub(1));
    line.push(b'\n');
    line
}
