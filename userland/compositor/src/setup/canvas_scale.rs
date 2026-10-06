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

//! Which screens get a half-size canvas. On its own so the clients that
//! scale their own drawing (first-boot setup, the installer) can be proved
//! against this exact rule never to scale a second time. The kernel's boot
//! console keeps the same rule in kernel_core/init/framebuffer/hidpi.rs, and
//! setup_layout_proofs holds the two to the same answer.

/// From here a panel is dense enough that one canvas pixel per screen pixel
/// draws everything too small. The desktop shell, setup and the installer
/// then scale their own drawing by the canvas they are given.
const HIDPI_MIN_WIDTH: u32 = 2560;
const HIDPI_MIN_HEIGHT: u32 = 1440;

/// Pixels of screen per pixel of canvas, each way.
pub fn scale_for(width: u32, height: u32) -> u32 {
    if width >= HIDPI_MIN_WIDTH && height >= HIDPI_MIN_HEIGHT {
        2
    } else {
        1
    }
}

/// GNOME's (mutter's) density for a 2x panel, in dots per inch.
pub const HIDPI_MIN_DPI: u64 = 192;

/// The smallest canvas the desktop, setup and the installer are laid out
/// for. A panel whose half would be smaller keeps one canvas pixel per
/// screen pixel, and the shell's own quarter steps (1.25, 1.5) size it.
const MIN_CANVAS_WIDTH: u32 = 1280;
const MIN_CANVAS_HEIGHT: u32 = 720;

/// A physical size worth believing: a real panel, not the aspect ratio some
/// projectors and TVs put in the size fields (mutter rejects the same).
fn plausible_mm(mm: Option<(u32, u32)>) -> Option<(u32, u32)> {
    let (w, h) = mm?;
    if w < 100 || h < 50 {
        return None;
    }
    if matches!((w, h), (160, 90) | (160, 100) | (1600, 900) | (1600, 1000)) {
        return None;
    }
    Some((w, h))
}

/// Pixels of screen per pixel of canvas for a panel whose EDID may give its
/// size in millimetres. With a size, a panel of 192 DPI or more each way is
/// doubled when its half still fits the smallest canvas, so a 13 inch
/// 2560x1600 doubles and a 27 inch 2560x1440 (109 DPI) does not. With no
/// size, the resolution rule above stands in.
pub fn scale_for_panel(width: u32, height: u32, mm: Option<(u32, u32)>) -> u32 {
    let Some((w_mm, h_mm)) = plausible_mm(mm) else {
        return scale_for(width, height);
    };
    if width / 2 < MIN_CANVAS_WIDTH || height / 2 < MIN_CANVAS_HEIGHT {
        return 1;
    }
    let dpi_x = width as u64 * 254 / (w_mm as u64 * 10);
    let dpi_y = height as u64 * 254 / (h_mm as u64 * 10);
    if dpi_x >= HIDPI_MIN_DPI && dpi_y >= HIDPI_MIN_DPI {
        2
    } else {
        1
    }
}
