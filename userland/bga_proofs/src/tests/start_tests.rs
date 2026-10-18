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

//! The mode bga starts on: firmware's when DISPI shows one, else the
//! monitor's preferred mode when it fits, else 1024x768.

use crate::dispi::{current, set_mode, DispiMode};
use crate::display::fits;
use crate::edid::Panel;
use crate::model::{blob, dtd, Adapter, VRAM};
use crate::setup::mode::starting;

/// Video memory and the BAR0 mapping, both the model's whole.
const ALL: (u64, u64) = (VRAM as u64, VRAM as u64);

fn panel(b: &[u8]) -> Panel {
    Panel::read(b, |w, h| fits(VRAM, w, h))
}

fn mode(w: u32, h: u32) -> Option<DispiMode> {
    Some(DispiMode { width: w, height: h, stride: w * 4 })
}

#[test]
fn the_mode_firmware_left_is_kept() {
    /*
     * Section 12: a driver keeps firmware's mode until a MODE_SET, so the
     * boot picture is not torn down by a driver that only came up.
     */
    let a = Adapter::new();
    set_mode(a.regs(), 800, 600);
    let p = panel(&blob(Some(dtd(1280, 800, (286, 179))), &[], 0));
    assert_eq!(starting(a.regs(), &p, ALL).ok(), mode(800, 600));
}

#[test]
fn an_adapter_left_off_starts_on_the_monitors_preferred_mode() {
    let a = Adapter::new();
    let p = panel(&blob(Some(dtd(1280, 800, (286, 179))), &[], 0));
    assert_eq!(starting(a.regs(), &p, ALL).ok(), mode(1280, 800));
    assert_eq!(current(a.regs()), mode(1280, 800));
}

#[test]
fn with_no_monitor_word_it_starts_at_1024x768() {
    let a = Adapter::new();
    assert_eq!(starting(a.regs(), &Panel::default(), ALL).ok(), mode(1024, 768));
}

#[test]
fn a_preferred_mode_too_large_for_video_memory_falls_back() {
    /*
     * A panel read with one budget and started with a smaller one: the
     * start checks again rather than trusting the panel's filter.
     */
    let a = Adapter::new();
    let p = panel(&blob(Some(dtd(1920, 1080, (531, 299))), &[], 0));
    assert_eq!(starting(a.regs(), &p, (4 << 20, 4 << 20)).ok(), mode(1024, 768));
}
