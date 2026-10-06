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

//! Every line first-boot setup draws fits the box it is drawn in, on every
//! canvas, measured in the brand's real faces at the layout's sizes.

use nonos_policy_proto::apps::{OPTIONAL, REQUIRED};
use nonos_policy_proto::keyboard_layout_labels::KEYBOARD_LAYOUT_LABELS;

use crate::displays::canvases;
use crate::literals::literals;
use crate::qwen_labels::LABELS;
use crate::wizard_layout::{column, layout, Layout, APPS_PURPOSE_AT, QWEN_CELLS, REVIEW_VALUE_AT};
use crate::wizard_theme::STEP_LABELS;
use crate::{label_w, measure, release, Face};

fn body(l: &Layout, s: &str) -> u32 {
    measure(s, Face::Body, l.body_px, 0.0)
}

fn caps(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}

/// A literal setup shows as keys in mono capitals: letters, none lower case.
fn is_keys(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_alphabetic()) && !s.chars().any(|c| c.is_ascii_lowercase())
}

fn every_canvas(mut check: impl FnMut(&str, &Layout, u32)) {
    for c in canvases() {
        let l = layout(c.width, c.height);
        let (_, col_w) = column(c.width, l.unit);
        check(&format!("{}x{}", c.width, c.height), &l, col_w);
    }
}

/// A face that failed to parse would measure every string as nothing and
/// pass every fit below; these are the real faces, measuring real widths.
#[test]
fn the_brand_faces_load_and_measure() {
    for face in [Face::Body, Face::Headline, Face::Mono] {
        let short = measure("NONOS", face, 17.0, 0.0);
        let long = measure("NONOS installs onto a disk", face, 17.0, 0.0);
        assert!(short > 30 && long > 3 * short, "{short} {long}");
        assert!(crate::line_h(face, 17.0) >= 17);
    }
    assert!(measure("NONOS", Face::Body, 34.0, 0.0) > measure("NONOS", Face::Body, 17.0, 0.0));
    assert!(label_w("ENTER NEXT", 14.0) > measure("ENTER NEXT", Face::Mono, 14.0, 0.0));
}

#[test]
fn every_step_fits_the_panel() {
    every_canvas(|at, l, _| {
        for name in STEP_LABELS {
            let name = caps(&String::from_utf8_lossy(name));
            let w = label_w("13", l.label_px) + l.unit + l.unit / 4 + label_w(&name, l.label_px);
            assert!(l.steps_x + w + l.unit <= l.panel_w, "{at}: step {name} is {w} wide");
        }
        for s in ["FIRST-BOOT SETUP".to_string(), release()] {
            let w = label_w(&s, l.label_px);
            assert!(w + 2 * l.unit <= l.panel_w, "{at}: {s} is {w} wide");
        }
    });
}

#[test]
fn every_line_setup_draws_fits_the_column() {
    let lines = literals("../capsule_setup_wizard/src", true);
    assert!(lines.len() > 150, "the scan found the screens' text");
    every_canvas(|at, l, col_w| {
        for lit in &lines {
            let t = &lit.text;
            let w = if is_keys(t) { label_w(t, l.caption_px) } else { body(l, t) };
            assert!(w <= col_w, "{at}: {} \"{t}\" is {w} wide, the column {col_w}", lit.file);
            // Anything short enough to be a list item or a title fits a row
            // and the title size.
            if t.chars().count() <= 40 && !is_keys(t) {
                assert!(w + 3 * l.unit <= l.list_w, "{at}: \"{t}\" overflows a row");
                let title = measure(t, Face::Headline, l.title_px, 0.0);
                assert!(title <= col_w, "{at}: \"{t}\" as a title is {title} wide");
            }
        }
    });
}

#[test]
fn text_put_together_at_run_time_fits_too() {
    let wifi_driver = literals("../nonos_wifi_client/src/driver", false);
    let wifi_saved = literals("../nonos_wifi_client/src/saved", false);
    every_canvas(|at, l, col_w| {
        let fits = |s: &str, room: u32| {
            let w = body(l, s);
            assert!(w <= room, "{at}: \"{s}\" is {w} wide, has {room}");
        };
        let stars: String = core::iter::repeat_n('*', 64).collect();
        fits(&format!("Passphrase: {stars}"), col_w);
        for lit in &wifi_driver {
            fits(&format!("Wi-Fi card: rtl8821ce: {}", lit.text), col_w);
        }
        for lit in &wifi_saved {
            fits(&format!("Cannot remember it: {}", lit.text), col_w);
        }
        let marker = label_w("STEP 13 OF 13", l.caption_px) + l.caption_px as u32;
        assert!(marker <= col_w, "{at}: the step marker");
        let scrolled = caps("rows 10 to 18 of 18, Up and Down scroll");
        assert!(label_w(&scrolled, l.caption_px) <= l.list_w, "{at}: the Qwen window line");
        // A 32 character network name in a row, and a 63 character computer
        // name beside its heading on the review.
        fits("Riverside-Library-Guest-Network5", l.list_w - 3 * l.unit);
        let host = "workstation-lab-02-second-floor-east-wing-near-the-print-room-x";
        assert_eq!(host.len(), 63);
        fits(host, col_w - REVIEW_VALUE_AT * l.unit);
    });
}

#[test]
fn columns_inside_rows_do_not_run_into_each_other() {
    every_canvas(|at, l, col_w| {
        let u = l.unit;
        for app in OPTIONAL {
            let name = format!("off  {}", String::from_utf8_lossy(app.name));
            let end = 2 * u + body(l, &name);
            assert!(end + u <= APPS_PURPOSE_AT * u, "{at}: {name} runs into its purpose");
            let purpose = String::from_utf8_lossy(app.purpose);
            let end = APPS_PURPOSE_AT * u + body(l, &purpose);
            assert!(end + u <= l.list_w, "{at}: {purpose} runs out of its row");
        }
        for line in REQUIRED {
            let w = 2 * u + body(l, &String::from_utf8_lossy(line));
            assert!(w <= col_w, "{at}: a required line");
        }
        for head in
            ["Keyboard", "Name", "Computer", "Time zone", "Network", "Wallpaper", "Qwen model"]
        {
            assert!(body(l, head) + u <= REVIEW_VALUE_AT * u, "{at}: {head} runs into its answer");
        }
        for name in KEYBOARD_LAYOUT_LABELS {
            let w = body(l, &String::from_utf8_lossy(name));
            assert!(w + 2 * u <= REVIEW_VALUE_AT * u + l.list_w, "{at}");
        }
        for (_, name) in LABELS {
            let name = String::from_utf8_lossy(name);
            let end = QWEN_CELLS[0] * u + body(l, &name);
            assert!(end + u <= QWEN_CELLS[1] * u, "{at}: {name} runs into its size");
        }
        let size = QWEN_CELLS[1] * u + body(l, "123.4 GB");
        assert!(size + u <= QWEN_CELLS[2] * u, "{at}: a size runs into its fit");
        for fit in ["needs 1.0 GB, on this stick", "needs 21.5 GB"] {
            let end = QWEN_CELLS[2] * u + body(l, fit);
            assert!(end <= l.list_w, "{at}: {fit} runs out of its row");
        }
    });
}
