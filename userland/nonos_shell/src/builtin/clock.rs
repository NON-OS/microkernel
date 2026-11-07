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

//! `date` and `uptime`, from the host's clocks. `date` writes the wall
//! clock as UTC, `YYYY-MM-DD HH:MM:SS UTC`, as the terminal's `date` does;
//! `uptime` writes `up Hh Mm Ss`, as the terminal's `uptime` does.

use alloc::vec::Vec;

use crate::host::Host;
use crate::out::ShellOut;
use crate::util::format_u64;

const DAY_S: u64 = 86_400;

pub fn date(out: &mut dyn ShellOut, host: &mut dyn Host) -> bool {
    let Some(ms) = host.wall_ms() else {
        out.writeln(b"date: clock unavailable");
        return false;
    };
    out.writeln(&utc(ms / 1000));
    true
}

pub fn uptime(out: &mut dyn ShellOut, host: &mut dyn Host) -> bool {
    let Some(ms) = host.uptime_ms() else {
        out.writeln(b"uptime: unavailable");
        return false;
    };
    let secs = ms / 1000;
    let mut line = b"up ".to_vec();
    num(&mut line, secs / 3600, 1);
    line.extend_from_slice(b"h ");
    num(&mut line, (secs % 3600) / 60, 1);
    line.extend_from_slice(b"m ");
    num(&mut line, secs % 60, 1);
    line.push(b's');
    out.writeln(&line);
    true
}

/// Seconds since the Unix epoch as a UTC date and time.
pub fn utc(secs: u64) -> Vec<u8> {
    let (y, m, d) = civil(secs / DAY_S);
    let t = secs % DAY_S;
    let mut s = Vec::with_capacity(24);
    num(&mut s, y, 4);
    s.push(b'-');
    num(&mut s, m, 2);
    s.push(b'-');
    num(&mut s, d, 2);
    s.push(b' ');
    num(&mut s, t / 3600, 2);
    s.push(b':');
    num(&mut s, (t % 3600) / 60, 2);
    s.push(b':');
    num(&mut s, t % 60, 2);
    s.extend_from_slice(b" UTC");
    s
}

/// The proleptic Gregorian date `days` after 1970-01-01.
fn civil(days: u64) -> (u64, u64, u64) {
    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z % 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + u64::from(m <= 2);
    (y, m, d)
}

fn num(out: &mut Vec<u8>, v: u64, width: usize) {
    let mut buf = [0u8; 20];
    let n = format_u64(v, &mut buf);
    out.extend(core::iter::repeat_n(b'0', width.saturating_sub(n)));
    out.extend_from_slice(&buf[..n]);
}
