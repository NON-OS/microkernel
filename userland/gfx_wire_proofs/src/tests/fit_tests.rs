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

//! The fit adapter over CAPS and MODES.

use crate::body::caps::{Caps, MODE_RANGE, MODE_SET};
use crate::body::mode::Mode;
use crate::fit::fit;

fn caps(flags: u32) -> Caps {
    Caps {
        kind: 2,
        outputs: 1,
        flags,
        max_width: 2560,
        max_height: 1600,
        width_align: 8,
        surface_budget: 2560 * 1600 * 4 + 64 * 64 * 4,
        physical_mm: 0,
    }
}

fn m(w: u32, h: u32) -> Mode {
    Mode { width: w, height: h, flags: 0 }
}

#[test]
fn a_range_adapter_takes_the_clamped_aligned_size() {
    let c = caps(MODE_SET | MODE_RANGE);
    assert_eq!(fit((1366, 768), (1024, 768), &c, &[]), Some((1360, 768)));
    assert_eq!(fit((3000, 2000), (1024, 768), &c, &[]), Some((2560, 1600)));
    assert_eq!(fit((1024, 768), (1024, 768), &c, &[]), None);
}

#[test]
fn a_list_adapter_takes_its_largest_mode_inside() {
    let c = caps(MODE_SET);
    let modes = [m(1024, 768), m(1920, 1080), m(1280, 720)];
    assert_eq!(fit((1900, 1200), (1024, 768), &c, &modes), Some((1280, 720)));
    assert_eq!(fit((1920, 1200), (1024, 768), &c, &modes), Some((1920, 1080)));
    assert_eq!(fit((0, 0), (1024, 768), &c, &modes), None);
}

#[test]
fn more_than_twenty_modes_are_cut_to_the_wires_twenty() {
    let mut modes: Vec<Mode> = (0..20).map(|i| m(640 + i, 480)).collect();
    modes.push(m(800, 600));
    assert_eq!(fit((800, 600), (640, 480), &caps(MODE_SET), &modes), Some((659, 480)));
}
