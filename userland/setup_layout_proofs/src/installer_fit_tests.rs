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

//! Every line the installer draws fits the box it is drawn in, full screen
//! on every canvas and in the window, measured in the brand's real faces.

use crate::displays::canvases;
use crate::installer::full::layout::layout;
use crate::installer::metrics::Metrics;
use crate::installer::text::keys_px;
use crate::installer_layout_tests::bodies;
use crate::literals::{literals, Literal};
use crate::{label_w, measure, Face};

fn caps(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}

fn is_keys(s: &str) -> bool {
    s.chars().any(|c| c.is_ascii_alphabetic()) && !s.chars().any(|c| c.is_ascii_lowercase())
}

fn ui_literals(dir: &str) -> Vec<Literal> {
    literals(&format!("../capsule_install/src/install/{dir}"), false)
}

/// A string the tests name by hand must still be in the source, or the test
/// would go on passing for text that no longer ships.
fn shipped(all: &[Literal], s: &str) {
    assert!(all.iter().any(|l| l.text == s), "\"{s}\" is not in the installer's source");
}

#[test]
fn every_title_and_subtitle_fits() {
    let titles = ui_literals("state");
    let subtitles: Vec<_> =
        ui_literals("ui/full").into_iter().filter(|l| l.file.ends_with("subtitle.rs")).collect();
    assert!(titles.len() >= 8 && subtitles.len() >= 8);
    for c in canvases() {
        let l = layout(c.width, c.height);
        let at = format!("{}x{}", c.width, c.height);
        for t in &titles {
            let w = measure(&t.text, Face::Headline, l.title_px, 0.0);
            assert!(w <= l.col_w, "{at}: title \"{}\" is {w} wide, has {}", t.text, l.col_w);
        }
        for s in &subtitles {
            let w = measure(&s.text, Face::Body, l.body_px, 0.0);
            assert!(w <= l.col_w, "{at}: \"{}\" is {w} wide", s.text);
        }
        let marker = label_w("STEP 05 OF 05", l.caption_px) + l.caption_px as u32;
        assert!(marker <= l.col_w, "{at}: the step marker");
        for s in ["INSTALL N\u{d8}NOS", "SETUP: NO ANSWERS KEPT", "04  WRITE, READ BACK"] {
            assert!(l.steps_x.min(2 * l.unit) + label_w(s, l.label_px) <= l.panel_w, "{at}: {s}");
        }
    }
    // The window's own header: the title beside the mark, the step at right.
    let m = Metrics::window();
    let room = crate::installer::metrics::WIN_W - 2 * m.pad - 6 * m.unit;
    for t in &titles {
        let w = measure(&t.text, Face::Headline, m.title_px, 0.0) + label_w("05 / 05", m.small_px);
        assert!(w + m.unit <= room, "the window's header: {}", t.text);
    }
}

/// The paragraphs the screens word-wrap; every_screen_fits_its_body checks
/// their wrapped height instead of a single line's width.
const WRAPPED: [&str; 8] = [
    "Installs the system",
    "The bootloader proved",
    "No driver is serving",
    "This list looks again",
    "Intel RST/VMD is on",
    "With it on, the disks",
    "It has a complete table",
    "The write stopped before",
];

#[test]
fn every_line_on_a_screen_fits_its_body() {
    let mut lines = ui_literals("ui/screens");
    lines.extend(ui_literals("ui/widgets"));
    assert!(lines.len() > 40, "the scan found the screens' text");
    for start in WRAPPED {
        assert!(lines.iter().any(|l| l.text.starts_with(start)), "{start}");
    }
    let wrapped = |l: &Literal| {
        l.file.ends_with("failed_text.rs") || WRAPPED.iter().any(|s| l.text.starts_with(s))
    };
    for b in bodies() {
        let m = &b.m;
        for lit in lines.iter().filter(|l| !wrapped(l)) {
            let t = &lit.text;
            let w = if is_keys(t) {
                label_w(t, m.label_px)
            } else {
                measure(t, Face::Body, m.body_px, 0.0)
            };
            assert!(w + m.inset <= b.w, "{}: {} \"{t}\" is {w}, has {}", b.at, lit.file, b.w);
        }
    }
}

#[test]
fn key_and_value_rows_keep_their_labels_in_their_column() {
    let all = ui_literals("ui/screens");
    let mine = [
        "TPM",
        "boot partition",
        "boot verdict",
        "bootloader",
        "disk",
        "firmware secure boot",
        "holds now",
        "kernel image",
        "kernel measurement",
        "read back",
        "size",
        "store",
        "write time",
        "written",
    ];
    for s in mine {
        shipped(&all, s);
    }
    // The plan's rows, from nonos_disk's describe.
    let plan = literals("../nonos_disk/src/describe", false);
    let theirs =
        ["erased", "table", "carried", "left out", "not carried", "disk plan", "data volume"];
    for s in theirs {
        assert!(plan.iter().any(|l| l.text == s), "\"{s}\" is not a plan row");
    }
    for b in bodies() {
        let m = &b.m;
        for s in mine.iter().chain(theirs.iter()) {
            let w = label_w(&caps(s), m.label_px);
            assert!(w + m.unit <= m.label_w, "{}: {s} runs into its value", b.at);
        }
        for caption in ["what will be written, and this machine", "what is erased and written"] {
            shipped(&all, caption);
            assert!(label_w(&caps(caption), m.label_px) + 2 * m.inset <= b.w, "{}", b.at);
        }
        // A disk's forty character model, with its size at the right.
        let model = "WDC WDS100T2B0C-00PXH0 NVMe 1TB SSD 0123";
        assert_eq!(model.len(), 40);
        let row = measure(model, Face::Body, m.body_px, 0.0)
            + measure("1000.2 GB", Face::Body, m.body_px, 0.0)
            + 3 * m.inset;
        assert!(row <= b.w, "{}: a disk's model runs into its size", b.at);
    }
}

/// Each screen's two hints, from hints.rs, fit side by side at the size the
/// footer picks for them.
#[test]
fn the_footer_hints_never_run_into_each_other() {
    let all = ui_literals("ui");
    let pairs = [
        ("Esc leave, the desktop starts", "Enter see the proofs"),
        ("Esc close", "Enter see the proofs"),
        ("Esc back", "Enter choose a disk"),
        ("Esc back", "R look again   \u{2191}\u{2193} select   Enter continue"),
        ("Esc back", "R look again"),
        ("Esc back", "type the word, then Enter"),
        ("Esc stop, the disk is left without a table", "do not power off"),
        ("", "writing the table, do not power off"),
        ("", "do not power off"),
        ("", "Enter restart now"),
        ("Esc close", "Enter restart now"),
        ("Esc leave, the desktop starts", "Enter choose another disk"),
        ("Esc close", "Enter choose another disk"),
    ];
    for (left, right) in pairs {
        for s in [left, right].into_iter().filter(|s| !s.is_empty()) {
            shipped(&all, s);
        }
    }
    let fits = |at: &str, room: u32, gap: u32, px: f32, small: f32| {
        for (left, right) in pairs {
            let size = keys_px(left, right, room, gap, px, small);
            let w = label_w(&caps(left), size) + label_w(&caps(right), size) + gap;
            assert!(w <= room, "{at}: \"{left}\" and \"{right}\" need {w}, have {room}");
        }
    };
    for c in canvases() {
        let l = layout(c.width, c.height);
        let at = format!("{}x{}", c.width, c.height);
        fits(&at, l.col_w, 2 * l.unit, l.caption_px, l.label_px);
    }
    let m = Metrics::window();
    let content = nonos_toolkit::decorations::content_rect(
        crate::installer::metrics::WIN_W,
        crate::installer::metrics::WIN_H,
        false,
    );
    fits("the window", content.w - 2 * m.pad, m.inset, m.small_px, m.label_px);
}
