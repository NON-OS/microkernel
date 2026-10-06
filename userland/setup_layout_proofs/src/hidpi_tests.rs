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

//! The HiDPI rule on real panels: a laptop's EDID size decides, the
//! resolution rule stands in without one, a doubled canvas is never smaller
//! than the layouts are proved at, and the kernel console agrees.

use crate::canvas_scale::{scale_for, scale_for_panel};
use crate::kernel_hidpi::hidpi_scale;

/// (width, height, width_mm, height_mm, expected) for panels in the field.
const PANELS: [(u32, u32, u32, u32, u32); 10] = [
    (1366, 768, 344, 194, 1),   // 15.6" HP 15s, Gemini Lake
    (1920, 1080, 344, 194, 1),  // 15.6" FHD, 141 DPI
    (1920, 1080, 294, 165, 1),  // 13.3" FHD, 166 DPI
    (2560, 1600, 286, 179, 2),  // 13.3" 16:10, 227 DPI
    (2560, 1600, 345, 215, 1),  // 16" 16:10, 188 DPI
    (2560, 1440, 597, 336, 1),  // 27" desktop monitor, 109 DPI
    (2256, 1504, 285, 190, 1),  // 13.5" 3:2: half would be 1128x752
    (2880, 1800, 302, 189, 2),  // 14" 16:10, 242 DPI
    (3840, 2160, 344, 194, 2),  // 15.6" UHD laptop, 283 DPI
    (3840, 2160, 597, 336, 1),  // 27" UHD monitor, 163 DPI
];

#[test]
fn a_panel_with_an_edid_size_is_judged_by_its_density() {
    for (w, h, wmm, hmm, want) in PANELS {
        assert_eq!(scale_for_panel(w, h, Some((wmm, hmm))), want, "{w}x{h} {wmm}x{hmm}mm");
    }
}

#[test]
fn without_a_size_the_resolution_rule_stands_in() {
    for (w, h) in [(1366, 768), (1920, 1080), (2560, 1440), (2560, 1600), (3840, 2160)] {
        assert_eq!(scale_for_panel(w, h, None), scale_for(w, h), "{w}x{h}");
    }
    assert_eq!(scale_for_panel(3840, 2160, None), 2, "4K doubles with no EDID");
    assert_eq!(scale_for_panel(1366, 768, None), 1);
}

#[test]
fn aspect_ratios_posing_as_sizes_are_ignored() {
    for mm in [(160, 90), (160, 100), (1600, 900), (16, 9), (0, 194), (344, 0)] {
        assert_eq!(scale_for_panel(2560, 1440, Some(mm)), scale_for(2560, 1440), "{mm:?}");
    }
}

#[test]
fn a_doubled_canvas_is_never_below_the_smallest_layout() {
    for w in (1024..=7680).step_by(32) {
        for h in (600..=4320).step_by(24) {
            for mm in [None, Some((200, 120)), Some((294, 165)), Some((600, 340))] {
                if scale_for_panel(w, h, mm) == 2 {
                    assert!(w / 2 >= 1280 && h / 2 >= 720, "{w}x{h} {mm:?}");
                }
            }
        }
    }
}

#[test]
fn the_kernel_console_takes_the_compositors_scale() {
    for w in (800..=7680).step_by(64) {
        for h in (600..=4320).step_by(40) {
            for mm in [None, Some((160, 90)), Some((286, 179)), Some((344, 194)), Some((700, 400))] {
                assert_eq!(hidpi_scale(w, h, mm), scale_for_panel(w, h, mm), "{w}x{h} {mm:?}");
            }
        }
    }
}
