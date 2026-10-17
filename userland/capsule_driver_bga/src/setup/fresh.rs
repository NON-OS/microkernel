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
//! The adapter as a driver finds it at bring-up, made its own.
//! L78 keeps a display function for its own driver service, reset or not,
//! so a restarted driver may meet what its killed self left: the last
//! frame in video memory, its mode still scanning out, or a mode set cut
//! off part way (DISPI disabled with its sizes half written). Video memory
//! is zeroed whole, and the zeros made visible, before any mode is touched,
//! so no mode this driver sets ever shows the old frame; then the starting
//! mode is chosen as section 12 says (`mode.rs`).

use super::mode::starting;
use crate::display::{fits, wipe, Bga};
use crate::edid::Panel;
use crate::regs::Regs;

/// The adapter at `regs` with `vram_len` bytes of video memory at `vram`
/// and the EDID `blob`, or None when no mode can be started.
pub fn fresh(regs: Regs, (vram, vram_len): (u64, usize), blob: &[u8], fence: fn()) -> Option<Bga> {
    // SAFETY: `vram` maps `vram_len` bytes, this driver's while it runs.
    unsafe { wipe(vram, vram_len) };
    fence();
    let panel = Panel::read(blob, |w, h| fits(vram_len, w, h));
    let mode = starting(regs, &panel, vram_len)?;
    Some(Bga { regs, vram, vram_len, panel, mode, generation: 1, fence })
}
