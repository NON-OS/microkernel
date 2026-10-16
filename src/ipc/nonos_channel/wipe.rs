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

//! IPC payloads zeroed before their memory is freed (C8). A message's
//! payload is zeroed wherever the message drops: delivered, refused by a
//! full or dead inbox, drained at exit. A payload copied in from a capsule
//! is held in a Payload, zeroed when it drops. No freed heap block keeps a
//! private message for the next allocation to read.
//! kernel_proofs::ipc_wipe.

extern crate alloc;

use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{compiler_fence, Ordering};

use super::message::IpcMessage;

/// Zero every byte `v` holds, to its capacity, so no store is elided as
/// dead before the free.
pub(super) fn wipe(v: &mut Vec<u8>) {
    let p = v.as_mut_ptr();
    for i in 0..v.capacity() {
        // SAFETY: `i` is within the allocation of `v`.
        unsafe { core::ptr::write_volatile(p.add(i), 0) };
    }
    compiler_fence(Ordering::SeqCst);
}

impl Drop for IpcMessage {
    fn drop(&mut self) {
        wipe(&mut self.data);
    }
}

/// `len` bytes for a copy from a capsule, zeroed when dropped.
pub struct Payload(Vec<u8>);

impl Payload {
    pub fn zeroed(len: usize) -> Self {
        Self(alloc::vec![0u8; len])
    }
}

impl Deref for Payload {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl DerefMut for Payload {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Drop for Payload {
    fn drop(&mut self) {
        wipe(&mut self.0);
    }
}
