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

//! The census looked at on init's tick, once a second, and said when
//! `when::say_now` says so.

use super::rule::{render, render_summary, rows, Row, Tally};
use super::when::{say_now, Look};
use crate::sys::serial::Line;
use spin::Mutex;

struct State {
    from: Option<u64>,
    said: Option<[Row; 11]>,
}

static STATE: Mutex<State> = Mutex::new(State { from: None, said: None });

/// Init has spawned everything: the census may be taken from now.
pub fn start(now_ms: u64) {
    STATE.lock().from.get_or_insert(now_ms);
}

pub fn tick(now_ms: u64) {
    let mut s = STATE.lock();
    let Some(from) = s.from else { return };
    let facts = super::gather::facts(now_ms);
    let now = rows(&facts);
    let look = Look {
        said: s.said.is_some(),
        changed: s.said.as_ref() != Some(&now),
        green: Tally::of(&now).green(),
        pass_done: crate::hardware::broker::acpi::first_pass::done() || !facts.acpi_capsule,
        waited_ms: now_ms.saturating_sub(from),
    };
    if say_now(look) {
        say(&now, now_ms.saturating_sub(from));
        s.said = Some(now);
    }
}

/// The census last said, as its lines (`MkBootCensus`); empty before it is.
pub fn text() -> alloc::vec::Vec<u8> {
    let mut v = alloc::vec::Vec::new();
    let Some(rows) = STATE.lock().said else { return v };
    for r in &rows {
        render(r, &mut |b| v.extend_from_slice(b));
        v.push(b'\n');
    }
    render_summary(&Tally::of(&rows), &mut |b| v.extend_from_slice(b));
    v.push(b'\n');
    v
}

fn say(rows: &[Row; 11], after_ms: u64) {
    Line::new().str(b"[CENSUS] taken ").dec(after_ms).str(b" ms after init's spawns").end();
    for r in rows {
        let mut l = Line::new();
        render(r, &mut |b| {
            l.str(b);
        });
        l.end();
    }
    let mut l = Line::new();
    render_summary(&Tally::of(rows), &mut |b| {
        l.str(b);
    });
    l.end();
}
