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

//! Giving the scanout memory back: both grants of a pair, and at the
//! driver's end the pair shown, any pair a mode set left, and the cursor's;
//! and the fence that orders the stores into it.

use nonos_libc::mk_dma_unmap;

use crate::dma_pair::Pair;
use crate::frames::DmaFrames;

/// Give both grants of `p` back.
pub fn unmap(p: &Pair) {
    for g in p.grant {
        let _ = mk_dma_unmap(g);
    }
}

/// The driver's end gives back the pair shown and any a mode set left.
impl Drop for DmaFrames {
    fn drop(&mut self) {
        unmap(&self.now);
        if let Some(w) = self.was.take() {
            unmap(&w);
        }
        if let Some((_, _, g)) = self.cursor.take() {
            let _ = mk_dma_unmap(g);
        }
    }
}

/// Order the write-combining stores before what follows.
pub fn sfence() {
    // SAFETY: sfence has no operands; it orders the WC stores before it.
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_mm_sfence()
    };
}
