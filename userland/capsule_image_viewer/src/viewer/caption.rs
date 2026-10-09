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

//! The words and numbers the single view's overlays show, kept apart from the
//! drawing so the proofs check what the window says.

extern crate alloc;
use alloc::format;
use alloc::string::String;

use crate::viewer::viewport::{base_scale, FitMode};

/// The help overlay, one row per key the single view answers to
/// (`app.rs` `on_key` and the pointer handlers).
pub const KEYMAP: &[(&str, &str)] = &[
    ("< >  <-/->", "prev/next"),
    ("scroll/+/-", "zoom"),
    ("drag", "pan / swipe"),
    ("f/1/w", "fit/actual/fill"),
    ("r/h/v", "rotate/flip-h/flip-v"),
    ("i/?", "info/help"),
    ("space", "slideshow"),
    ("[ / ]", "slideshow faster/slower"),
    ("0", "reset zoom and pan"),
    ("Esc/Bksp", "back to gallery"),
];

/// The scale the image is drawn at, as a percentage of its own pixels. The
/// view's zoom is relative to the fit mode's scale, so a photo fitted to the
/// window at a third of its size is 33%, not the 100% the zoom factor reads.
pub fn zoom_percent(mode: FitMode, iw: u32, ih: u32, vw: u32, vh: u32, zoom: f32) -> u32 {
    let pct = base_scale(mode, iw, ih, vw, vh) * zoom * 100.0;
    if pct <= 0.0 {
        0
    } else {
        (pct + 0.5) as u32
    }
}

/// The slideshow's interval: whole seconds as "3s", a half step as "3.5s".
pub fn interval_label(ms: u64) -> String {
    let tenths = (ms + 50) / 100;
    if tenths.is_multiple_of(10) {
        format!("{}s", tenths / 10)
    } else {
        format!("{}.{}s", tenths / 10, tenths % 10)
    }
}

/// The info overlay's second line: size, format and file size, or that the
/// file was not shown, since a failed open has no image to measure.
pub fn size_line(dims: Option<(u32, u32)>, fmt: &str, bytes: u64) -> String {
    match dims {
        Some((w, h)) => format!("{}x{}  {}  {}B", w, h, fmt, bytes),
        None => format!("not shown  {}", fmt),
    }
}

/// Whether the prev and next buttons are drawn, and so whether a click there
/// steps: with more than one image to step to, shown or not, since a file
/// that failed to decode is the one most worth stepping past.
pub fn nav_shown(dir_len: usize) -> bool {
    dir_len > 1
}
