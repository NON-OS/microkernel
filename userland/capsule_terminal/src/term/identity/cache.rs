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

use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use nonos_policy_proto::Field;

use super::fetch;

const CAP: usize = 64;

/*
 * One policy string, asked for once per capsule.
 *
 * `on_enter` runs on the input path and repaints run every frame, so reading
 * one must never be an IPC round trip after the first: a dead policy server
 * would otherwise cost a timeout per keystroke.
 */
pub struct Cached {
    field: Field,
    keep: fn(&[u8]) -> usize,
    buf: UnsafeCell<[u8; CAP]>,
    len: AtomicUsize,
    asked: AtomicBool,
}

/*
 * Safety: a capsule is single threaded and the cell is written exactly once,
 * by the first `get`, before `len` publishes the length any reader uses.
 */
unsafe impl Sync for Cached {}

impl Cached {
    /* `keep` says how much of a reply is safe to draw, counting from the front. */
    pub const fn new(field: Field, keep: fn(&[u8]) -> usize) -> Self {
        Self {
            field,
            keep,
            buf: UnsafeCell::new([0u8; CAP]),
            len: AtomicUsize::new(0),
            asked: AtomicBool::new(false),
        }
    }

    /* The value policy holds, or nothing when it holds none or cannot be reached. */
    pub fn get(&'static self) -> &'static [u8] {
        if !self.asked.swap(true, Ordering::Relaxed) {
            let mut raw = [0u8; CAP];
            let got = fetch::string(self.field, &mut raw).unwrap_or(0);
            let n = (self.keep)(&raw[..got]);
            let slot: &mut [u8; CAP] = unsafe { &mut *self.buf.get() };
            slot[..n].copy_from_slice(&raw[..n]);
            self.len.store(n, Ordering::Release);
        }
        let n = self.len.load(Ordering::Acquire);
        let slot: &'static [u8; CAP] = unsafe { &*self.buf.get() };
        &slot[..n]
    }
}
