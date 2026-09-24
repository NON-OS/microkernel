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

//! The round key schedule an AES-128 instance holds, and its widths.

pub const BLOCK_BYTES: usize = 16;
pub const KEY_BYTES: usize = 16;
pub(crate) const ROUNDS: usize = 10;
pub(crate) const EXPANDED_BYTES: usize = BLOCK_BYTES * (ROUNDS + 1);

pub struct Aes128 {
    pub(crate) round_keys: [u8; EXPANDED_BYTES],
}

impl Drop for Aes128 {
    fn drop(&mut self) {
        for byte in self.round_keys.iter_mut() {
            /*
             * SAFETY: eK@nonos.systems. A plain assignment to a field of a
             * value being dropped is dead code the optimiser may remove, and
             * this is a key schedule. The write must survive, so it goes
             * through a volatile store to an address that is live here.
             */
            unsafe { core::ptr::write_volatile(byte, 0) };
        }
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
