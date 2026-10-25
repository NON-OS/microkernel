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
//! The words for known modes, worked out by hand from Vol 2c's field rules.

use crate::model::mode::{cta_1080p, dmt_800x600};
use nonos_intel_xelp_display::timing::check::Sink;
use nonos_intel_xelp_display::timing::words::words;

#[test]
fn cta_1080p_words() {
    let w = words(&cta_1080p(), Sink::Hdmi).unwrap();
    // Total 2200 and active 1920, less one each: 2199 << 16 | 1919.
    assert_eq!(w.htotal, (2199 << 16) | 1919);
    assert_eq!(w.hblank, (2199 << 16) | 1919);
    // Sync from 1920 + 88 - 1 = 2007 to 1920 + 88 + 44 - 1 = 2051.
    assert_eq!(w.hsync, (2051 << 16) | 2007);
    assert_eq!(w.vtotal, (1124 << 16) | 1079);
    assert_eq!(w.vblank, (1124 << 16) | 1079);
    // 1080 + 4 - 1 = 1083 to 1080 + 4 + 5 - 1 = 1088.
    assert_eq!(w.vsync, (1088 << 16) | 1083);
    assert_eq!(w.srcsz, (1919 << 16) | 1079);
}

#[test]
fn dmt_800x600_words() {
    let w = words(&dmt_800x600(), Sink::DisplayPort).unwrap();
    assert_eq!(w.htotal, (1055 << 16) | 799);
    assert_eq!(w.hsync, (967 << 16) | 839);
    assert_eq!(w.vtotal, (627 << 16) | 599);
    // A one-line front porch: sync starts at line 600, field value 600.
    assert_eq!(w.vsync, (604 << 16) | 600);
    assert_eq!(w.srcsz, (799 << 16) | 599);
}
