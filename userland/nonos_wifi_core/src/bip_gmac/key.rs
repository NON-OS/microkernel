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

//! The 256-bit IGTK (or BIGTK) a BIP-GMAC-256 link holds, with its key id
//! (IEEE Std 802.11-2020, 12.7.1.4: 4 or 5 for an IGTK, 6 or 7 for a BIGTK).
//! Kept apart from the 16-octet BIP-CMAC-128 key so neither can be handed
//! to the other's MIC by mistake. The key is overwritten when dropped.

use core::ptr::write_volatile;
use core::sync::atomic::{compiler_fence, Ordering};

pub struct BipGmacKey {
    id: u16,
    key: [u8; 32],
}

impl BipGmacKey {
    /// `None` for a key id outside 4..=7, which no IGTK or BIGTK uses.
    pub fn new(id: u16, key: [u8; 32]) -> Option<Self> {
        (4..=7).contains(&id).then_some(Self { id, key })
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub(super) fn bytes(&self) -> &[u8; 32] {
        &self.key
    }
}

impl Drop for BipGmacKey {
    fn drop(&mut self) {
        for b in self.key.iter_mut() {
            // Volatile so the compiler cannot drop the store as dead.
            unsafe { write_volatile(b, 0) };
        }
        compiler_fence(Ordering::SeqCst);
    }
}
