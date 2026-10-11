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

//! The card as bring-up left it.

use super::ext_csd::{ExtCsd, CMD6_MIN_MS};
use super::regs::Cid;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rca: u16,
    /// The OCR the card answered its final CMD1 with.
    pub ocr: u32,
    /// Addresses are sector numbers (OCR access mode 10b), else bytes.
    pub sector_mode: bool,
    pub cid: Cid,

    /// None on a card older than CSD SPEC_VERS 4, which has no EXT_CSD.
    pub ext: Option<ExtCsd>,
    /// The user data area in 512-byte sectors.
    pub sectors: u64,
    /// Multi-block transfers are preceded by SET_BLOCK_COUNT. Every card of
    /// MMC 3.1 or later has CMD23; Linux uses it on every MMC card.
    pub cmd23: bool,
    pub width: u8,
    pub hs: bool,
    pub hz: u32,
}

impl Card {
    /// The card holds writes in a volatile cache until FLUSH_CACHE.
    pub fn cache_on(&self) -> bool {
        self.ext.is_some_and(|e| e.cache_on())
    }

    /// How long a SWITCH of `index` may keep the card busy.
    pub fn switch_ms(&self, index: u8) -> u64 {
        self.ext.map_or(CMD6_MIN_MS, |e| e.switch_ms(index))
    }
}
