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

//! `seq`: a column of integers, the counterpart a `for` loop or a pipe feeds.
//!
//! `seq LAST`, `seq FIRST LAST`, and `seq FIRST STEP LAST`, one number per
//! line. The count is capped so a slipped bound cannot flood the scrollback;
//! past the cap it stops and says so rather than run without end.

use crate::out::ShellOut;
use crate::util::format_u64;

/// The most rows `seq` will print before it refuses to go on.
const MAX_ROWS: u64 = 100_000;

/// Emit the sequence. Returns false on a bad argument or an overrun, so the
/// status reflects whether the whole run printed.
pub fn run(out: &mut dyn ShellOut, argv: &[&[u8]]) -> bool {
    let nums = argv.get(1..).unwrap_or(&[]);
    let (first, step, last) = match nums.len() {
        1 => (Some(1), Some(1), parse(nums[0])),
        2 => (parse(nums[0]), Some(1), parse(nums[1])),
        3 => (parse(nums[0]), parse(nums[1]), parse(nums[2])),
        _ => {
            out.writeln(b"usage: seq [first [step]] last");
            return false;
        }
    };
    let (Some(first), Some(step), Some(last)) = (first, step, last) else {
        out.writeln(b"seq: arguments must be whole numbers");
        return false;
    };
    if step == 0 {
        out.writeln(b"seq: step may not be zero");
        return false;
    }
    emit(out, first, step, last)
}

/// Walk from `first` to `last` by `step`, up or down, bounded by `MAX_ROWS`.
fn emit(out: &mut dyn ShellOut, first: i64, step: i64, last: i64) -> bool {
    let mut value = first;
    let mut rows = 0u64;
    while (step > 0 && value <= last) || (step < 0 && value >= last) {
        if rows == MAX_ROWS {
            out.writeln(b"seq: stopped at 100000 lines");
            return false;
        }
        write_line(out, value);
        rows += 1;
        value = match value.checked_add(step) {
            Some(v) => v,
            None => break,
        };
    }
    true
}

fn write_line(out: &mut dyn ShellOut, v: i64) {
    let mut buf = [0u8; 21];
    let mut n = 0;
    if v < 0 {
        buf[0] = b'-';
        n = 1;
    }
    n += format_u64(v.unsigned_abs(), &mut buf[n..]);
    out.writeln(&buf[..n]);
}

/// A base-ten integer with an optional sign, or None for anything else.
fn parse(arg: &[u8]) -> Option<i64> {
    let (neg, digits) = match arg.first() {
        Some(b'-') => (true, &arg[1..]),
        Some(b'+') => (false, &arg[1..]),
        _ => (false, arg),
    };
    if digits.is_empty() || !digits.iter().all(u8::is_ascii_digit) {
        return None;
    }
    let mut v: i64 = 0;
    for &b in digits {
        v = v.checked_mul(10)?.checked_add((b - b'0') as i64)?;
    }
    Some(if neg { -v } else { v })
}
