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

//! Which mode to drive: the firmware's when this driver can serve it, so the
//! boot's screen stays up; else its own.

use super::mode::{Choice, Mode};
use crate::constants::{
    BYTES_PER_PIXEL, DISPI_BPP_32, DISPI_ENABLED, DISPI_LFB_ENABLED, MODE_HEIGHT, MODE_WIDTH,
};

/// The DISPI registers as found.
#[derive(Clone, Copy, Debug, Default)]
pub struct Seen {
    pub enable: u16,
    pub xres: u16,
    pub yres: u16,
    pub bpp: u16,
    pub virt_width: u16,
    pub x_offset: u16,
    pub y_offset: u16,
}

/// `None` when not even the driver's own mode fits in `vram` bytes.
pub fn choose(seen: Seen, vram: u64) -> Option<Choice> {
    let on = DISPI_ENABLED | DISPI_LFB_ENABLED;
    let kept = Mode {
        width: seen.xres as u32,
        height: seen.yres as u32,
        stride: seen.virt_width as u32 * BYTES_PER_PIXEL,
    };
    // A panned or narrower-than-shown mode is not one a single copy serves.
    let usable = seen.enable & on == on
        && seen.bpp == DISPI_BPP_32
        && seen.xres > 0
        && seen.yres > 0
        && seen.virt_width >= seen.xres
        && seen.x_offset == 0
        && seen.y_offset == 0;
    if usable && kept.bytes() <= vram {
        return Some(Choice::Keep(kept));
    }
    let own = Mode { width: MODE_WIDTH, height: MODE_HEIGHT, stride: MODE_WIDTH * BYTES_PER_PIXEL };
    (own.bytes() <= vram).then_some(Choice::Set(own))
}
