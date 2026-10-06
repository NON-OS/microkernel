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

//! The step list: every stage the loader runs, numbered on thin rules like
//! the proofs panel, with its state on the right, under a headline that says
//! what the loader is doing now and a bar of the steps done.

use core::sync::atomic::{AtomicU8, Ordering};

use super::layout::{splash, STEPS};
use super::stage::StageStatus;
use crate::display::fx::clear_region;
use crate::display::gop::{hline, is_initialized};
use crate::display::ink::palette::{BAD, BORDER, CYAN, TEXT, TEXT_2, TEXT_3};
use crate::display::ink::{
    draw, draw_captions, label, label_width, metrics, round_rect, Style,
};
use crate::display::version::version_label;

const PENDING: u8 = 0;
const RUNNING: u8 = 1;
const DONE: u8 = 2;
const FAILED: u8 = 3;

/* Each stage's state, by stage number; 0 (init) is not listed. */
static STATE: [AtomicU8; STEPS as usize + 1] = [const { AtomicU8::new(PENDING) }; STEPS as usize + 1];

/// What the row names: the step, as a person reads it.
fn name(stage: usize) -> &'static [u8] {
    match stage {
        1 => b"Firmware and display",
        2 => b"Platform security",
        3 => b"Hardware and memory",
        4 => b"Load the kernel",
        5 => b"Measure the kernel, BLAKE3",
        6 => b"Kernel signature, Ed25519 and ML-DSA-65",
        7 if cfg!(feature = "dev-attest") => b"Kernel Merkle path, development",
        7 => b"Kernel STARK proof",
        8 => b"Read the kernel image",
        9 => b"Hand off to the kernel",
        _ => b"Start N\xD8NOS",
    }
}

/// The headline while a step runs.
fn doing(stage: usize) -> &'static [u8] {
    match stage {
        1 => b"Starting the display",
        2 => b"Checking platform security",
        3 => b"Reading the hardware",
        4 => b"Loading the kernel",
        5 => b"Measuring the kernel",
        6 => b"Checking the kernel's signature",
        7 if cfg!(feature = "dev-attest") => b"Checking the kernel's path",
        7 => b"Checking the kernel's proof",
        8 => b"Reading the kernel image",
        9 => b"Handing off to the kernel",
        _ => b"Starting N\xD8NOS",
    }
}

/// Record a stage's state and redraw what it changes.
pub fn set(stage: u8, status: StageStatus) {
    let i = stage as usize;
    if i == 0 || i > STEPS as usize {
        return;
    }
    let v = match status {
        StageStatus::Pending => PENDING,
        StageStatus::Running => RUNNING,
        StageStatus::Success => DONE,
        StageStatus::Failed => FAILED,
    };
    STATE[i].store(v, Ordering::Relaxed);
    if !is_initialized() {
        return;
    }
    draw_row(i);
    draw_head();
    if i == STEPS as usize && v == DONE {
        let s = splash();
        let (x, y, w, h) = s.s.frame;
        let mono = metrics(Style::Mono);
        clear_region(x, y + h + mono.line * 2, w, mono.line * 2 + s.u);
        draw_captions(&s.s, b"VERIFIED", CYAN, version_label().as_bytes(), TEXT_3);
    }
}

fn state(i: usize) -> u8 {
    STATE[i].load(Ordering::Relaxed)
}

/// The headline, the bar and every row.
pub fn draw_steps() {
    if !is_initialized() {
        return;
    }
    draw_head();
    for i in 1..=STEPS as usize {
        draw_row(i);
    }
    hline(splash().col_x, row_top(STEPS as usize + 1), splash().col_w, BORDER);
}

fn row_top(i: usize) -> u32 {
    let s = splash();
    s.list_y + s.row_h * (i as u32 - 1)
}

/* The headline says the step that runs, the one that failed, or that all
are done; the bar fills with the steps done. */
fn draw_head() {
    let s = splash();
    let head = metrics(Style::Heading);
    let all = (1..=STEPS as usize).filter(|&i| state(i) == DONE).count() as u32;
    let failed = (1..=STEPS as usize).find(|&i| state(i) == FAILED);
    let running = (1..=STEPS as usize).rev().find(|&i| state(i) == RUNNING);
    let (text, color): (&[u8], u32) = match (failed, running) {
        (Some(_), _) => (b"The boot stopped", BAD),
        (None, _) if all == STEPS => (b"Verified. Starting N\xD8NOS", TEXT),
        (None, Some(i)) => (doing(i), TEXT),
        (None, None) => (b"Verifying N\xD8NOS", TEXT),
    };
    clear_region(s.col_x, s.title_y, s.col_w, head.line);
    draw(s.col_x, s.title_y, text, Style::Heading, color);
    round_rect(s.col_x, s.bar_y, s.col_w, 2, 1, BORDER);
    let fill = (s.col_w * all / STEPS).max(2);
    round_rect(s.col_x, s.bar_y, fill, 2, 1, if failed.is_some() { BAD } else { CYAN });
}

/* One row: its number, its name, and its state word on the right. */
fn draw_row(i: usize) {
    let s = splash();
    let (u, body, mono) = (s.u, metrics(Style::Body), metrics(Style::Mono));
    let top = row_top(i);
    clear_region(s.col_x, top, s.col_w, s.row_h);
    hline(s.col_x, top, s.col_w, BORDER);
    let y = top + u;
    let num = [b'0' + (i / 10) as u8, b'0' + (i % 10) as u8];
    let my = y + body.ascent.saturating_sub(mono.ascent);
    let st = state(i);
    let (word, wc, nc, numc): (&[u8], u32, u32, u32) = match st {
        DONE => (b"\x01 DONE", CYAN, TEXT_2, TEXT_3),
        RUNNING => (b"CHECKING", TEXT, TEXT, CYAN),
        FAILED => (b"\x02 FAILED", BAD, BAD, BAD),
        _ => (b"", TEXT_3, TEXT_3, BORDER),
    };
    label(s.col_x, my, &num, numc);
    draw(s.col_x + label_width(b"00") + 3 * u, y, name(i), Style::Body, nc);
    if !word.is_empty() {
        label(s.col_x + s.col_w - label_width(word), my, word, wc);
    }
}
