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

//! The refusal screen, in the brand's language: the frame in red with the Ø
//! unlit, and numbered sections saying what happened, what to do, and the
//! loader's own reason.

use super::advice::Advice;
use crate::display::fx::fill_atmosphere;
use crate::display::gop::hline;
use crate::display::ink::palette::{BAD, BORDER, TEXT, TEXT_2, TEXT_3};
use crate::display::ink::{draw_captions, draw_emblem, draw_wrapped, label};
use crate::display::ink::{lines, marker, metrics, scene, Scene, Style};

/// Draw the screen; returns the scene, whose footer the countdown uses.
pub fn draw_card(a: &Advice, reason: &[u8]) -> Scene {
    fill_atmosphere();
    let probe = scene(0);
    let (u, cw) = (probe.u, probe.col_w);
    let (mono, body, head) = (metrics(Style::Mono), metrics(Style::Body), metrics(Style::Heading));
    let n = |s: &[u8], st: Style| lines(s, cw, st);
    let block = mono.line + 3 * u + head.line * n(a.title, Style::Heading) + 8 * u;
    let sections = 3 * (mono.line + 2 * u + 6 * u);
    let text = body.line * (n(a.what, Style::Body) + n(a.remedy, Style::Body))
        + mono.line * n(reason, Style::Mono);
    let s = scene(block + sections + text);
    draw_emblem(&s, 1000, BAD, false);
    draw_captions(&s, b"REFUSED", BAD, b"NOTHING RAN", TEXT_3);
    let mut ty = s.col_y;
    marker(s.col_x, ty, b"BOOT REFUSED", BAD, BAD);
    ty =
        draw_wrapped(s.col_x, ty + mono.line + 3 * u, cw, a.title, Style::Heading, TEXT, 3) + 8 * u;
    ty = section(&s, ty, b"01", b"WHAT HAPPENED", a.what, Style::Body, TEXT_2);
    ty = section(&s, ty, b"02", b"WHAT TO DO", a.remedy, Style::Body, TEXT);
    section(&s, ty, b"03", b"THE LOADER'S REASON", reason, Style::Mono, TEXT_3);
    s
}

fn section(s: &Scene, y: u32, num: &[u8], name: &[u8], text: &[u8], st: Style, c: u32) -> u32 {
    let u = s.u;
    hline(s.col_x, y, s.col_w, BORDER);
    let ly = y + 2 * u;
    let nw = label(s.col_x, ly, num, BAD) + 3 * u;
    label(s.col_x + nw, ly, name, TEXT_3);
    let ty = ly + metrics(Style::Mono).line + 2 * u;
    draw_wrapped(s.col_x, ty, s.col_w, text, st, c, 6) + 4 * u
}
