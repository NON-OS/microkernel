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

/*
 * What the Qwen step holds: this machine's memory, how many tiers fit it,
 * and the chosen row. The tiers are smallest first, so the ones that fit
 * are the first `fit` of them, and only those rows can be chosen.
 */

use super::fit::fits;
use super::memory::memory;
use super::pins::PINNED;

pub struct QwenState {
    /* Bytes of memory; None when the kernel would not say, and then no tier fits. */
    pub memory: Option<u64>,
    pub fit: u8,
    /* Row: 0 is "None for now", 1 and on are PINNED. */
    pub sel: u8,
}

impl QwenState {
    /* Starts on the largest Qwen3 tier that fits, or on none when none does. */
    pub fn read() -> Self {
        let memory = memory();
        let fit = PINNED.iter().take_while(|(_, b)| memory.is_some_and(|m| fits(*b, m))).count();
        let qwen3 = PINNED[..fit].iter().rposition(|(t, _)| t.starts_with(b"qwen3-"));
        Self { memory, fit: fit as u8, sel: qwen3.map_or(0, |i| i as u8 + 1) }
    }

    /* The chosen tier's name, as the Terminal's qwen takes it. */
    pub fn chosen(&self) -> Option<&'static [u8]> {
        let i = (self.sel as usize).checked_sub(1)?;
        PINNED.get(i).map(|(tier, _)| *tier)
    }
}
