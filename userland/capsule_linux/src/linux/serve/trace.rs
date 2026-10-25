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

//! The diagnostic chain's head for this capsule's family, kept as each
//! answered call is noted and handed to the kernel through MkSealNote slot 1
//! once every EVERY answered calls, at the start of the next trap: a note on
//! every call would tax every guest call for a diagnostic. The head the seal
//! keeps may so miss up to EVERY - 1 of the last links. One serve thread
//! steps it.

use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering::Relaxed};

use super::trace_link::{label, link};

const DIAG: u64 = 1;
const EVERY: u32 = 16;

static HEAD: [AtomicU64; 4] = [const { AtomicU64::new(0) }; 4];
static UNSERVED: AtomicBool = AtomicBool::new(false);
/// Links folded in since the head was last handed over.
static PENDING: AtomicU32 = AtomicU32::new(0);

fn head() -> [u8; 32] {
    let mut b = [0u8; 32];
    for (o, w) in b.chunks_exact_mut(8).zip(&HEAD) {
        o.copy_from_slice(&w.load(Relaxed).to_le_bytes());
    }
    b
}

/// The call being answered has no answer here: its link says so.
pub fn unserved() {
    UNSERVED.store(true, Relaxed);
}

/// One answered call, folded in.
pub fn step(nr: u64, result: u64) {
    let (l, errno) = label(result, UNSERVED.swap(false, Relaxed));
    let next = link(&head(), nr, l, errno);
    for (w, c) in HEAD.iter().zip(next.chunks_exact(8)) {
        w.store(u64::from_le_bytes(c.try_into().unwrap_or([0; 8])), Relaxed);
    }
    PENDING.fetch_add(1, Relaxed);
}

/// The head, to the kernel, once EVERY links have been folded in since.
pub fn flush() {
    if PENDING.load(Relaxed) >= EVERY {
        PENDING.store(0, Relaxed);
        let _ = nonos_libc::mk_seal_note(DIAG, &head());
    }
}
