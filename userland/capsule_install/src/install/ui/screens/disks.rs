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

//! The disks, one row each: bus, size, and what is on it now. A driver
//! that did not answer, or a controller no driver serves, gets a row too,
//! in the fault colour, so the list says everything the machine has
//! rather than everything that worked. With nothing to install to, the
//! screen says why when it can tell, and that it is looking again.

use nonos_app_skeleton::PaintBuffer;

use super::disk_row::row;
use crate::install::state::State;
use crate::install::survey::watching;
use crate::install::ui::frame::Body;
use crate::install::ui::text::top_of;
use crate::install::ui::widgets::section;
use crate::install::ui::wrap::Ink;
use crate::install::ui::{text, theme};

const NO_DISK: &str = "No driver is serving a disk yet, so there is nowhere to install for now.";
const CONNECT: &str = "This list looks again every two seconds, and R looks now: a disk whose driver comes up late appears here without a restart. If none does, check that an NVMe or SATA disk is fitted and enabled in the firmware setup.";
const RAID: &str = "Intel RST/VMD is on: set the BIOS storage mode to AHCI (or turn VMD off), then boot this stick again.";
const RAID_WHY: &str = "With it on, the disks sit behind a RAID controller NONOS has no driver for. A Windows already on this computer may need switching to AHCI first, or it will not start after the change.";
const WATCHING: &str = "Looking again every two seconds. R looks now.";
const NOTE: &str =
    "The disk you choose is erased completely. The stick you booted from is not in this list.";

pub fn paint(state: &State, fb: &mut PaintBuffer, b: Body) {
    let m = &b.m;
    let ink = Ink::body(m, theme::FOREGROUND);
    if state.disks.is_empty() {
        let red = theme::DANGER;
        if state.raid {
            let y = section(fb, m, b.x, b.y, b.w, ("01", "STORAGE MODE", red), RAID, ink);
            section(fb, m, b.x, y, b.w, ("02", "WHY", red), RAID_WHY, ink);
        } else {
            let y = section(fb, m, b.x, b.y, b.w, ("01", "NO DISK", red), NO_DISK, ink);
            section(fb, m, b.x, y, b.w, ("02", "WHAT TO DO", red), CONNECT, ink);
        }
        return;
    }
    let mut y = b.y;
    for (i, d) in state.disks.iter().enumerate() {
        let selected = i == state.selected;
        if selected {
            fb.fill_round(b.x, y, b.w, m.row_h, m.radius, theme::SELECTED_BG);
            let bar = m.scale.px(3);
            fb.fill_rect(b.x, y + m.unit + m.unit / 2, bar, m.row_h - 3 * m.unit, theme::ACCENT);
        }
        row(fb, m, d, b.x, y, b.w);
        y += m.row_h + m.unit;
    }
    if state.raid {
        section(fb, m, b.x, y + m.unit, b.w, ("!", "STORAGE MODE", theme::WARN), RAID, ink);
    }
    let at = b.y + b.h.saturating_sub(m.line_h);
    if watching(state) {
        let above = at.saturating_sub(m.line_h);
        text::line(fb, b.x, top_of(above, m.line_h, m.small_px), WATCHING, theme::WARN, m.small_px);
    }
    text::line(fb, b.x, top_of(at, m.line_h, m.small_px), NOTE, theme::MUTED, m.small_px);
}
