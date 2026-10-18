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

//! The rules a data phase must keep: a command that carries data, the
//! transfer mode its block count needs, 512-byte blocks, and a count that
//! agrees with CMD23 and with what the card moves.

use super::sim::Sim;
use super::types::Xfer;

impl Sim {
    /// Hold the registers to the command; the bytes the phase moves.
    pub(super) fn check_data(&mut self, x: &Xfer, multi: bool) -> usize {
        let Xfer { index, mode, blocks, blksz, card_reads, ref bytes, .. } = *x;
        if !self.dev_ref().carries_data(index) {
            self.violate(format!("CMD{index} sent with data"));
        }
        let had_count = self.dev_ref().count().is_some() || !multi;
        let want_mode = 1
            | 2
            | if card_reads { 0x10 } else { 0 }
            | if multi { 0x20 } else { 0 }
            | if multi && !had_count { 0x04 } else { 0 };
        if mode != want_mode {
            self.violate(format!("CMD{index} transfer mode {mode:#x}, want {want_mode:#x}"));
        }
        if blksz & 0xfff != 512 {
            self.violate(format!("CMD{index} block size {blksz:#x}"));
        }
        let n = blocks as usize;
        if !multi && n != 1 {
            self.violate(format!("CMD{index} single block with count {n}"));
        }
        if let Some(c) = self.dev_ref().count() {
            if c as usize != n {
                self.violate(format!("CMD23 count {c}, block count {n}"));
            }
        }
        let len = n * 512;
        if card_reads && bytes.len() != len {
            self.violate(format!("CMD{index} reads {} bytes, host expects {len}", bytes.len()));
        }
        len
    }
}
