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

//! First-boot setup's layout on every canvas: every region on the canvas,
//! in order and apart, every space a whole number of units, and every
//! screen's content above the keys.

use crate::displays::{canvases, Canvas};
use crate::wizard_layout::{column, layout, most_body, window, Layout, QWEN_AFTER_LINES, STEPS};
use crate::{line_h, Face};

fn each() -> Vec<(Canvas, Layout)> {
    canvases().into_iter().map(|c| (c, layout(c.width, c.height))).collect()
}

#[test]
fn every_region_lies_on_the_canvas_in_order() {
    for (c, l) in each() {
        let at = format!("{}x{}", c.width, c.height);
        let u = l.unit;
        let (col_x, col_w) = column(c.width, u);
        assert_eq!(col_x, l.col_x, "{at}");
        assert!(l.panel_w + 8 * u <= col_x, "{at}: the column clears the panel");
        assert!(col_x + col_w + 8 * u <= c.width, "{at}: the column keeps its right margin");
        assert!(l.list_w <= col_w, "{at}: lists stay in the column");

        // The panel: the emblem's glow, the captions and every step inside it.
        let glow = (l.emblem_w / 24).max(3);
        assert!(l.emblem_x >= glow && l.emblem_y >= glow, "{at}: the emblem's glow is on screen");
        assert!(l.emblem_x + l.emblem_w + glow <= l.panel_w, "{at}: the emblem is in the panel");
        assert!(l.emblem_y + l.emblem_w * 10 / 9 + glow <= l.caption_y, "{at}");
        let label = line_h(Face::Mono, l.label_px);
        assert!(l.caption_y + label <= l.release_y, "{at}: caption over the release");
        assert!(l.release_y + label <= l.steps_y, "{at}: release over the steps");
        assert!(l.step_h >= label, "{at}: a step row holds its label");
        assert!(l.steps_y + STEPS * l.step_h <= c.height, "{at}: the last step is on screen");
        assert!(l.steps_x >= 2 * u, "{at}: the current step's bar is on screen");

        // The column, top to bottom: marker, title, subtitle, content, keys.
        let caption = line_h(Face::Mono, l.caption_px);
        assert!(l.marker_y >= 6 * u, "{at}");
        assert!(l.marker_y + caption <= l.title_y, "{at}: marker over the title");
        assert!(l.title_y + line_h(Face::Headline, l.title_px) <= l.sub_y, "{at}: title");
        assert!(l.sub_y + line_h(Face::Body, l.body_px) <= l.body_y, "{at}: subtitle");
        assert!(l.body_y < l.body_end && l.body_end < l.foot_y, "{at}");
        assert!(l.keys_y + caption <= c.height, "{at}: the keys are on screen");
        assert!(l.foot_y < l.keys_y, "{at}: the keys sit under their rule");
        assert!(line_h(Face::Body, l.body_px) <= l.line_h, "{at}: a body line fits its step");
    }
}

#[test]
fn every_space_is_a_whole_number_of_units() {
    for (c, l) in each() {
        let u = l.unit;
        for (name, v) in [
            ("marker", l.marker_y),
            ("title", l.title_y),
            ("subtitle", l.sub_y),
            ("content", l.body_y),
            ("row", l.row_h),
            ("line", l.line_h),
            ("gap", l.gap),
            ("step", l.step_h),
            ("list", l.list_w),
            ("from the bottom to the keys' rule", c.height - l.foot_y),
        ] {
            assert_eq!(v % u, 0, "{}x{}: {name} is {v}, not a multiple of {u}", c.width, c.height);
        }
    }
}

/// Rows grow to five units only where the tallest screen still fits.
#[test]
fn rows_are_five_units_where_there_is_room_and_four_where_not() {
    let want = [
        ((1366, 768), 4),
        ((1440, 900), 5),
        ((1920, 1080), 5),
        ((1280, 720), 4),
        ((1280, 800), 5),
        ((2560, 1440), 5),
        ((2560, 1600), 5),
        ((3840, 2160), 5),
    ];
    for ((w, h), units) in want {
        let l = layout(w, h);
        assert_eq!(l.row_h, units * l.unit, "{w}x{h}");
    }
}

/// What each screen puts under its title at most: list rows, body lines and
/// gaps between them, as its draw function lays them out.
const SCREENS: [(&str, u32, u32, u32); 12] = [
    // As many rows as the keyboard drivers have layouts.
    ("keyboard", nonos_keymap::POLICY_LAYOUTS.len() as u32, 0, 0),
    // The name as typed; the three rules; why the last key was refused.
    ("name", 1, 4, 2),
    ("time zone", 1, 0, 0),
    // Three modes; the install mode's six lines.
    ("mode", 3, 6, 1),
    // No network and six heard; the driver, the passphrase and its rule, what
    // the row does, WPA3, remembering, the wired card.
    ("network", 7, 7, 1),
    // Three routes; three lines on the route, a blank, two on changing it.
    ("route", 3, 6, 1),
    ("privacy", 0, 8, 0),
    ("appearance", 6, 0, 0),
    // Every optional app; "Required", its three, and three on what off means.
    ("apps", 8, 7, 2),
    ("installed software", 2, 0, 0),
    // The name as typed; four rules; why it was refused.
    ("computer name", 1, 5, 2),
    // Seven answers; six lines on what is kept.
    ("review", 0, 13, 1),
];

#[test]
fn every_screen_fits_above_the_keys() {
    for (c, l) in each() {
        let room = l.body_end - l.body_y;
        for (name, rows, lines, gaps) in SCREENS {
            // A list's closing rule is drawn in the gap after it.
            let need = rows * l.row_h + lines * l.line_h + gaps * l.gap;
            assert!(need <= room, "{}x{}: {name} needs {need}, has {room}", c.width, c.height);
        }
        let tallest = most_body(l.row_h / l.unit) * l.unit;
        assert!(tallest <= room, "{}x{}: the tallest screen", c.width, c.height);
    }
}

/// The Qwen list is eighteen rows: on a short canvas it shows a run of them
/// around the chosen row, and what it shows still fits.
#[test]
fn the_qwen_list_shows_what_fits_and_keeps_the_choice_in_view() {
    let rows = crate::qwen_labels::LABELS.len() as u32 + 1;
    for (c, l) in each() {
        let room = l.line_rows(QWEN_AFTER_LINES, 2);
        assert!(room >= 8, "{}x{}: only {room} Qwen rows", c.width, c.height);
        let (_, shown) = window(rows, room, 0);
        let need = shown * l.line_h + QWEN_AFTER_LINES * l.line_h + 2 * l.gap;
        assert!(need <= l.body_end - l.body_y, "{}x{}: Qwen needs {need}", c.width, c.height);
    }
    assert_eq!(rows, 18);
    for room in 1..=20 {
        for sel in 0..rows {
            let (first, shown) = window(rows, room, sel);
            assert_eq!(shown, rows.min(room));
            assert!(first <= sel && sel < first + shown, "room {room}, row {sel}");
            assert!(first + shown <= rows);
        }
    }
}

#[test]
fn a_taller_canvas_gives_the_title_more_room_above() {
    let small = layout(1280, 720);
    let large = layout(1920, 1080);
    assert!(large.marker_y > small.marker_y);
    assert!(large.body_end - large.body_y > small.body_end - small.body_y);
}
