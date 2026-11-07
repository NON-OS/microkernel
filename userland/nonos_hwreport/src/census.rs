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

//! The kernel's boot census as the report carries it: each `[CENSUS]` line
//! of the boot log, read field by field (src/hardware/boot_census/rule.rs
//! writes them) and written as
//!
//! ```text
//! census <class> <up|absent|missing|held> <expected> <up>[ <why>]
//! census summary <up> <absent> <held> <missing> <green|red>
//! ```
//!
//! A line that does not read as one of these is not carried.

use crate::sink::Sink;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Up,
    Absent,
    Missing,
    Held,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row<'a> {
    pub class: &'a [u8],
    pub state: State,
    pub expected: u8,
    pub up: u8,
    pub why: Option<&'a [u8]>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Summary {
    pub up: u8,
    pub absent: u8,
    pub held: u8,
    pub missing: u8,
    pub green: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Census<'a> {
    Row(Row<'a>),
    Summary(Summary),
}

const TAG: &[u8] = b"[CENSUS] ";

fn word<'a>(rest: &mut &'a [u8]) -> &'a [u8] {
    let end = rest.iter().position(|b| *b == b' ').unwrap_or(rest.len());
    let (w, r) = rest.split_at(end);
    *rest = r.strip_prefix(b" ").unwrap_or(r);
    w
}

fn number(w: &[u8], key: &[u8]) -> Option<u8> {
    let digits = w.strip_prefix(key)?;
    if digits.is_empty() || digits.len() > 3 || !digits.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let n = digits.iter().fold(0u16, |n, d| n * 10 + u16::from(d - b'0'));
    u8::try_from(n).ok()
}

fn state(w: &[u8]) -> Option<State> {
    Some(match w {
        b"up" => State::Up,
        b"absent" => State::Absent,
        b"missing" => State::Missing,
        b"held" => State::Held,
        _ => return None,
    })
}

fn summary(mut rest: &[u8]) -> Option<Summary> {
    let up = number(word(&mut rest), b"up=")?;
    let absent = number(word(&mut rest), b"absent=")?;
    let held = number(word(&mut rest), b"held=")?;
    let missing = number(word(&mut rest), b"missing=")?;
    let green = match word(&mut rest) {
        b"verdict=green" => true,
        b"verdict=red" => false,
        _ => return None,
    };
    (rest.is_empty() && green == (missing == 0)).then_some(Summary {
        up,
        absent,
        held,
        missing,
        green,
    })
}

/// One boot log line, if it is a census line.
pub fn parse(line: &[u8]) -> Option<Census<'_>> {
    let line = line.strip_suffix(b"\n").unwrap_or(line);
    let mut rest = line.strip_prefix(TAG)?;
    let class = word(&mut rest);
    if class == b"summary" {
        return summary(rest).map(Census::Summary);
    }
    if class.is_empty() || !class.iter().all(u8::is_ascii_lowercase) {
        return None;
    }
    let state = state(word(&mut rest))?;
    let expected = number(word(&mut rest), b"expected=")?;
    let up = number(word(&mut rest), b"up=")?;
    let why = match rest {
        [] => None,
        r => Some(r.strip_prefix(b"why=")?).filter(|w| !w.is_empty()),
    };
    if (state == State::Up) != why.is_none() {
        return None;
    }
    Some(Census::Row(Row { class, state, expected, up, why }))
}

fn state_word(s: State) -> &'static [u8] {
    match s {
        State::Up => b"up",
        State::Absent => b"absent",
        State::Missing => b"missing",
        State::Held => b"held",
    }
}

/// The report's `census` line for one boot log line; false when it is not
/// a census line or does not fit.
pub fn line(text: &[u8], out: &mut Sink) -> bool {
    match parse(text) {
        Some(Census::Row(r)) => {
            out.bytes(b"census ").bytes(r.class).bytes(b" ").bytes(state_word(r.state));
            out.bytes(b" ").dec(u64::from(r.expected)).bytes(b" ").dec(u64::from(r.up));
            if let Some(why) = r.why {
                out.bytes(b" ").bytes(why);
            }
        }
        Some(Census::Summary(s)) => {
            out.bytes(b"census summary ").dec(u64::from(s.up)).bytes(b" ");
            out.dec(u64::from(s.absent)).bytes(b" ").dec(u64::from(s.held)).bytes(b" ");
            out.dec(u64::from(s.missing)).bytes(if s.green { b" green" } else { b" red" });
        }
        None => return false,
    }
    out.bytes(b"\n");
    !out.full()
}
