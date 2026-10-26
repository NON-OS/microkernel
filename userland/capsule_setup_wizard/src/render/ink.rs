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

//! Setup's text in the brand's faces (nonos_brand), with the signature the
//! bitmap font had. Bytes are Latin-1, so 0xD8 still reads as the Ø. Every
//! size comes from the canvas's layout, so a canvas the compositor could not
//! halve gets its type scaled with everything else.

use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;
use nonos_brand::{label, line_h, marker, text, Face};

use super::canvas::Canvas;
use super::layout::layout;

fn latin1(s: &[u8]) -> String {
    s.iter().map(|&b| b as char).collect()
}

fn surface<'a>(cv: &'a mut Canvas) -> PaintBuffer<'a> {
    PaintBuffer { pixels: cv.buf, stride_words: cv.spx as u32, width: cv.w, height: cv.h }
}

/// Body text with its line box's top at `y`.
pub fn draw_text(cv: &mut Canvas, x: u32, y: u32, s: &[u8], c: u32) {
    let px = layout(cv.w, cv.h).body_px;
    let mut fb = surface(cv);
    text(&mut fb, x, y, &latin1(s), Face::Body, c, px);
}

/// A title: Geist at 500, `px` high.
pub fn draw_title(cv: &mut Canvas, x: u32, y: u32, s: &[u8], c: u32, px: f32) {
    let mut fb = surface(cv);
    text(&mut fb, x, y, &latin1(s), Face::Headline, c, px);
}

/// A mono label in capitals, tracked, at caption size; returns its right edge.
pub fn draw_label(cv: &mut Canvas, x: u32, y: u32, s: &[u8], c: u32) -> u32 {
    let px = layout(cv.w, cv.h).caption_px;
    let mut fb = surface(cv);
    let caps: String = s.iter().map(|&b| (b as char).to_ascii_uppercase()).collect();
    label(&mut fb, x, y, &caps, c, px)
}

/// The brand's section marker: a dot in `lamp`, then a caption-size label.
pub fn draw_marker(cv: &mut Canvas, x: u32, y: u32, s: &str, lamp: u32, c: u32) {
    let px = layout(cv.w, cv.h).caption_px;
    let mut fb = surface(cv);
    marker(&mut fb, x, y, s, lamp, c, px);
}

/// The height of one line of body text on this canvas.
pub fn body_line(w: u32, h: u32) -> u32 {
    line_h(Face::Body, layout(w, h).body_px)
}
