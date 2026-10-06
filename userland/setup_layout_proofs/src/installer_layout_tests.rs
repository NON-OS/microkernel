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

//! The installer's layout, full screen on every canvas and as a window: the
//! regions in order and apart, and every screen's body, as its paint lays it
//! out with the real word wrap, inside the body it is handed.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::decorations::content_rect;

use crate::displays::canvases;
use crate::installer::full::layout::{layout, FullLayout, STEPS};
use crate::installer::metrics::{confirm_stack, Metrics, MOST_BODY, WIN_H, WIN_W};
use crate::installer::wrap::{paragraph, Ink};
use crate::literals::literals;
use crate::{line_h, Face};

/// A screen's body: where it is drawn, and at what sizes.
pub struct Body {
    pub at: String,
    pub w: u32,
    pub h: u32,
    pub m: Metrics,
}

/// The body on every full-screen canvas, and in the window at its size.
pub fn bodies() -> Vec<Body> {
    let mut out: Vec<Body> = canvases()
        .into_iter()
        .map(|c| {
            let l = layout(c.width, c.height);
            Body { at: format!("{}x{}", c.width, c.height), w: l.col_w, h: l.body_h, m: l.m }
        })
        .collect();
    let c = content_rect(WIN_W, WIN_H, false);
    let m = Metrics::window();
    let (_, _, w, h) = m.window_body(c.w, c.h);
    out.push(Body { at: "the window".into(), w, h, m });
    out
}

/// Lines `s` wraps to at width `w`, by the installer's own wrap.
pub fn wrapped(m: &Metrics, w: u32, s: &str) -> u32 {
    let mut px = vec![0u32; (w * 2000) as usize];
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: 2000 };
    paragraph(&mut fb, 0, 0, w, s, Ink::body(m, 0xFFFF_FFFF)) / m.line_h
}

/// The literal in `file` under the installer's ui that starts with `start`.
pub fn text(file: &str, start: &str) -> String {
    let all = literals("../capsule_install/src/install", false);
    all.into_iter()
        .find(|l| l.file.ends_with(file) && l.text.starts_with(start))
        .unwrap_or_else(|| panic!("{file} has no text starting {start}"))
        .text
}

fn each() -> Vec<(String, u32, u32, FullLayout)> {
    canvases()
        .into_iter()
        .map(|c| {
            (format!("{}x{}", c.width, c.height), c.width, c.height, layout(c.width, c.height))
        })
        .collect()
}

#[test]
fn every_region_lies_on_the_canvas_in_order() {
    for (at, width, height, l) in each() {
        let u = l.unit;
        assert!(l.panel_w + 8 * u <= l.col_x, "{at}: the column clears the panel");
        assert!(l.col_x + l.col_w + 8 * u <= width, "{at}: the column keeps its margin");
        let glow = (l.emblem_w / 24).max(3);
        assert!(l.emblem_x >= glow && l.emblem_y >= glow, "{at}: the emblem is on screen");
        assert!(l.emblem_x + l.emblem_w + glow <= l.panel_w, "{at}: the emblem is in the panel");
        assert!(l.emblem_y + l.emblem_w * 10 / 9 + glow <= l.caption_y, "{at}");
        let label = line_h(Face::Mono, l.label_px);
        assert!(l.caption_y + label <= l.release_y && l.release_y + label <= l.steps_y, "{at}");
        assert!(l.steps_y + STEPS * l.step_h <= height, "{at}: the last step is on screen");
        assert!(l.step_h >= label && l.steps_x >= 2 * u, "{at}");

        let caption = line_h(Face::Mono, l.caption_px);
        assert!(l.marker_y + caption <= l.title_y, "{at}: marker over the title");
        assert!(l.title_y + line_h(Face::Headline, l.title_px) <= l.sub_y, "{at}: the title");
        assert!(l.sub_y + line_h(Face::Body, l.body_px) <= l.body_y, "{at}: the subtitle");
        assert!(l.body_y + l.body_h < l.foot_y, "{at}: the body ends above the keys' rule");
        assert!(l.foot_y < l.keys_y && l.keys_y + caption <= height, "{at}: the keys");
        for v in [l.marker_y, l.title_y, l.sub_y, l.body_y, height - l.foot_y, l.step_h] {
            assert_eq!(v % u, 0, "{at}: {v} is not a whole number of units");
        }
    }
}

#[test]
fn the_body_holds_the_tallest_screen_and_the_title_steps_down_only_where_it_must() {
    for (at, _, height, l) in each() {
        assert!(l.body_h >= MOST_BODY * l.unit, "{at}: body {} high", l.body_h);
        let compact = l.title_px < l.scale.font(crate::scale::TITLE_PX);
        assert_eq!(compact, height / l.unit < 92, "{at}: the title is stepped down");
    }
    let c = content_rect(WIN_W, WIN_H, false);
    let m = Metrics::window();
    let (_, _, _, h) = m.window_body(c.w, c.h);
    assert!(h >= MOST_BODY * m.unit, "the window's body is {h} high");
}

/// Every screen's content, as its paint lays it out, fits its body: the
/// longest text each can show, wrapped by the installer's own wrap.
#[test]
fn every_screen_fits_its_body() {
    let welcome = text("welcome.rs", "Installs the system");
    let proofs = text("proofs.rs", "The bootloader proved");
    let no_disk = text("disks.rs", "No driver is serving");
    let connect = text("disks.rs", "This list looks again");
    let raid = text("disks.rs", "Intel RST/VMD is on");
    let raid_why = text("disks.rs", "With it on, the disks");
    let table = text("failed.rs", "It has a complete table");
    let retries = literals("../capsule_install/src/install/ui/screens", false);
    let retry = retries
        .iter()
        .filter(|l| l.file.ends_with("failed_text.rs"))
        .map(|l| l.text.clone())
        .max_by_key(|t| t.len())
        .expect("failed_text.rs");
    let why = "stopped for a reason the writer did not name";
    let now = format!("{table} 1023.9 GB were written before it stopped.");
    for b in bodies() {
        let (m, w) = (&b.m, b.w);
        let lines = |s: &str| wrapped(m, w, s) * m.line_h;
        let section = |s: &str| 4 * m.unit + m.unit / 2 + lines(s) + m.gap;
        let screens = [
            ("welcome", lines(&welcome) + m.gap + m.card_h(6)),
            ("proofs", lines(&proofs) + m.gap + 5 * m.proof_row_h() + 1),
            ("disks", 3 * (m.row_h + m.unit) + 2 * m.line_h),
            ("disks behind RST", 2 * (m.row_h + m.unit) + m.unit + section(&raid) + 2 * m.line_h),
            ("no disk", section(&no_disk) + section(&connect)),
            ("no disk, RST", section(&raid) + section(&raid_why)),
            ("confirm", confirm_stack(m, 9)[3]),
            ("writing", 2 * m.line_h + 6 * m.unit + m.bar_h + 2 * m.unit + 2 * m.line_h),
            ("done", m.line_h + m.gap + m.card_h(7) + m.gap + 2 * m.line_h),
            ("failed", section(why) + section(&now) + section(&retry)),
        ];
        for (name, need) in screens {
            assert!(need <= b.h, "{}: the {name} screen needs {need}, has {}", b.at, b.h);
        }
    }
}

/// The confirm screen's blocks follow each other and the word's field ends
/// where the screen says it does.
#[test]
fn the_confirm_screen_stacks_in_order() {
    let m = Metrics::window();
    for rows in 0..=9 {
        let [plan, ask, field, end] = confirm_stack(&m, rows);
        assert_eq!(plan, m.card_h(4) + m.gap);
        assert_eq!(ask, plan + m.card_h(rows) + m.gap);
        assert!(field >= ask + m.line_h && end == field + m.field_h);
    }
    assert_eq!(confirm_stack(&m, 9)[3], MOST_BODY * m.unit, "nine rows are the tallest");
}
