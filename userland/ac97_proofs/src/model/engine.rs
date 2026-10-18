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

//! The engine: it fetches the descriptor at the current index through the
//! list address, plays the samples it names and moves on, halting on the
//! last valid entry, as QEMU's transfer_audio does.

use nonos_libc::dma_at;

use super::ich::Ich;

impl Ich {
    /// The next index becomes current and its descriptor is fetched.
    pub(super) fn advance(&mut self) {
        self.civ = self.piv;
        self.piv = (self.piv + 1) % 32;
        let at = dma_at(self.bdbar as u64 + self.civ as u64 * 8) as *const u32;
        // SAFETY: `dma_at` found the descriptor inside a live grant.
        let (addr, ctl) = unsafe { (u32::from_le(*at), u32::from_le(*at.add(1))) };
        assert_eq!(ctl & (1 << 31), 0, "no completion interrupt is asked for");
        assert_ne!(ctl & (1 << 30), 0, "a halted engine sends zeros");
        self.cur = addr & !3;
        self.picb = ctl as u16;
    }

    /// Play `bytes` of output time. A halted engine plays nothing. An entry
    /// played out moves the engine on at once, or halts it on the last
    /// valid one, so the current index always names what plays next.
    pub fn play(&mut self, mut bytes: usize) {
        while bytes >= 2 && self.sr & 1 == 0 && self.cr & 1 != 0 {
            let n = (self.picb as usize * 2).min(bytes);
            // SAFETY: `dma_at` found the period inside a live grant; the
            // descriptor never names more than its period.
            let src = unsafe { std::slice::from_raw_parts(dma_at(self.cur as u64), n) };
            if self.open() {
                self.heard.extend_from_slice(src);
            } else {
                self.muted_bytes += n;
            }
            self.cur += n as u32;
            self.picb -= (n / 2) as u16;
            bytes -= n;
            if self.picb == 0 && self.civ == self.lvi {
                self.sr |= 1 | 2 | 4;
                self.halts += 1;
            } else if self.picb == 0 {
                self.advance();
            }
        }
    }
}
