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

//! The mode bga starts on: firmware's when DISPI already shows a linear
//! 32-bit mode (section 12: a driver keeps the mode firmware left until a
//! MODE_SET), else the monitor's preferred mode when it fits, else 1024x768.

use crate::constants::FALLBACK_MODE;
use crate::dispi::{current, set_mode, DispiMode};
use crate::display::fits;
use crate::edid::Panel;
use crate::regs::Regs;

pub fn starting(regs: Regs, panel: &Panel, vram_len: usize) -> Option<DispiMode> {
    if let Some(m) = current(regs) {
        return Some(m);
    }
    let (w, h) = panel.preferred.unwrap_or(FALLBACK_MODE);
    let (w, h) = if fits(vram_len, w, h) { (w, h) } else { FALLBACK_MODE };
    set_mode(regs, w as u16, h as u16);
    current(regs)
}
