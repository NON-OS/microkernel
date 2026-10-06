// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Saying so when a guest's `..` meets the root.
//!
//! Clamping is what keeps the guest inside /linux, and it is silent by
//! nature: the path resolves, just not where the guest aimed. A program that
//! climbs above its root is either confused or trying to leave, and either
//! way the refusal belongs in the log.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_libc::mk_debug;

/// Enough to show an attempt; a guest looping on it cannot flood the console.
const LOGGED: u32 = 16;

static SEEN: AtomicU32 = AtomicU32::new(0);

pub(super) fn note(path: &[u8]) {
    if SEEN.fetch_add(1, Ordering::Relaxed) >= LOGGED {
        return;
    }
    let mut line = [0u8; 128];
    let head = b"[LINUX] refused: path above the root, clamped: ";
    line[..head.len()].copy_from_slice(head);
    let n = path.len().min(line.len() - head.len() - 1);
    line[head.len()..head.len() + n].copy_from_slice(&path[..n]);
    line[head.len() + n] = b'\n';
    let _ = mk_debug(line.as_ptr(), head.len() + n + 1);
}
