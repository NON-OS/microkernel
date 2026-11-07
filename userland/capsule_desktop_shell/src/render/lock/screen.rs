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

//! The lock screen on every output: the time, the date, the field and what
//! the last attempt came to.

use alloc::format;
use alloc::string::String;

use nonos_libc::mk_uptime_ms;

use super::super::dialog_kit::fill;
use super::super::dialog_text::line;
use crate::render::layout::Rect;
use crate::render::measure_aa::measure_aa;
use crate::render::text_aa::text_aa;
use crate::render::ui_font::{px, TITLE_PX, UI_PX};
use crate::state::indicators::clock_stamp::{stamp, STAMP_LEN};
use crate::state::lock::geometry::places;
use crate::state::lock::{words, Note, Phase};
use crate::state::Context;

/// Opaque: nothing under it shows through.
const CURTAIN: u32 = 0xFF07_0B10;
const FG: u32 = 0xFFDF_EAF7;
const DIM: u32 = 0xFFAF_BED2;
const WARN: u32 = 0xFFE8_B75C;
const TIME_PX: f32 = 64.0;

/// Paint the lock screen when it is up; true when it was.
pub fn paint(ctx: &Context) -> bool {
    if !ctx.lock.screen.covers() {
        return false;
    }
    fill(ctx, Rect { x: 0, y: 0, width: ctx.width, height: ctx.height }, CURTAIN);
    let mut buf = [0u8; STAMP_LEN];
    let n = stamp(&mut buf, ctx.readings.clock_24h, ctx.readings.tz_hours).unwrap_or(0);
    let when = core::str::from_utf8(&buf[..n]).unwrap_or("");
    // "Mon 02 Nov  07:42" and " 7:42 PM": the date first, then the time.
    let (date, time) = (when.get(..10).unwrap_or(""), when.get(12..).unwrap_or("").trim());
    let note = note_text(ctx);
    let whole = (0, 0, ctx.width, ctx.height);
    let outputs: &[(u32, u32, u32, u32)] =
        if ctx.lock.outputs.is_empty() { core::slice::from_ref(&whole) } else { &ctx.lock.outputs };
    for &o in outputs {
        one(ctx, o, (date, time), &note);
    }
    true
}

fn one(ctx: &Context, o: (u32, u32, u32, u32), (date, time): (&str, &str), note: &(String, u32)) {
    let p = places(o, px(1));
    let centre = |text: &str, size: f32| o.0 + o.2.saturating_sub(measure_aa(text, size)) / 2;
    let label = if time.is_empty() { words::LOCKED } else { time };
    text_aa(ctx, centre(label, TIME_PX), p.time_y, label, FG, TIME_PX);
    if !date.is_empty() {
        text_aa(ctx, centre(date, TITLE_PX), p.date_y, date, DIM, TITLE_PX);
    }
    let live = ctx.lock.screen.phase() == Phase::Locked;
    let chars = ctx.lock.screen.field.chars();
    super::field::paint(ctx, p.field, chars, words::FIELD_HINT, live);
    let (text, colour) = (note.0.as_str(), note.1);
    if !text.is_empty() {
        line(ctx, centre(text, UI_PX), p.note_y, text, colour, UI_PX, o.2);
    }
}

/// What to say under the field, and in what colour.
fn note_text(ctx: &Context) -> (String, u32) {
    let s = &ctx.lock.screen;
    if s.phase() == Phase::Checking {
        return (String::from(words::CHECKING), DIM);
    }
    let wait = s.wait_left(mk_uptime_ms());
    let wrong = match s.note() {
        Note::Wrong { failures } if failures > 1 => format!("{failures} {}", words::WRONG_MANY),
        Note::Wrong { .. } => String::from(words::WRONG_ONE),
        Note::Unavailable => return (String::from(words::NO_ANSWER), WARN),
        Note::None => String::new(),
    };
    if wait > 0 {
        let secs = wait.div_ceil(1000);
        let w = format!("{} {secs} {}", words::WAIT_BEFORE, words::SECONDS);
        return (if wrong.is_empty() { w } else { format!("{wrong}. {w}") }, WARN);
    }
    (wrong, WARN)
}
