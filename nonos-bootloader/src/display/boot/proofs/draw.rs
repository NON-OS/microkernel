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

//! The panel, in the splash's column: each proof a numbered row, its name,
//! its state in mono on the right, and what backs it in mono underneath.

use super::geometry::{frame, Frame};
use super::rows::{Row, ROWS};
use crate::display::fx::clear_region;
use crate::display::gop::hline;
use crate::display::ink::palette::{BORDER, CYAN, TEXT, TEXT_2, TEXT_3};
use crate::display::ink::{draw, label, label_width, marker, metrics, width, Style};

pub fn draw_panel(rows: &[Row; ROWS]) -> Frame {
    let f = frame();
    clear_region(f.x, f.y, f.w, f.h);
    marker(f.x, f.y, b"WHAT THIS BOOT VERIFIED", CYAN, TEXT_2);
    for (i, row) in rows.iter().enumerate() {
        draw_row(&f, i, row);
    }
    let end = f.row_y(ROWS);
    hline(f.x, end, f.w, BORDER);
    let legend: &[u8] = b"VERIFIED: BY THIS LOADER   PRESENT: BY THE KERNEL";
    label(f.x, end + 2 * f.u, legend, TEXT_3);
    f
}

fn draw_row(f: &Frame, i: usize, row: &Row) {
    let (u, mono) = (f.u, metrics(Style::Mono));
    let top = f.row_y(i);
    hline(f.x, top, f.w, BORDER);
    let y = top + u + u / 2;
    let num = [b'0', b'1' + i as u8];
    label(f.x, y + u / 2, &num, TEXT_3);
    draw(f.x + f.label_x, y, screen_name(row.label), Style::Body, TEXT);
    let word = row.state.word();
    label(f.x + f.w - label_width(word), y + u / 2, word, row.state.color());
    let d = row.detail.as_bytes();
    let room = f.w.saturating_sub(f.detail_x);
    let mut n = d.len();
    while n > 0 && width(&d[..n], Style::Mono) > room {
        n -= 1;
    }
    let dy = y + metrics(Style::Body).line + u / 2;
    draw(f.x + f.detail_x, dy.min(top + f.row_h - mono.line), &d[..n], Style::Mono, TEXT_2);
}

/// The boot log keeps its upper-case labels; the screen uses these.
fn screen_name(label: &[u8]) -> &[u8] {
    match label {
        b"KERNEL" if cfg!(feature = "dev-attest") => b"Kernel path (dev)",
        b"KERNEL" => b"Kernel STARK",
        b"KERNEL SIGNATURE" => b"Kernel signature",
        b"BOOTLOADER" => b"Loader trailer",
        b"BOOT-ROOT RECORD" => b"Boot-root record",
        b"TCG LOG" => b"TCG event log",
        b"SECURE BOOT" => b"Secure Boot",
        _ => label,
    }
}
