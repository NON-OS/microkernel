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

/*
 * /dev/nonos-metrics: numbers a guest reports about its own run, said in
 * the log as numbers and nothing else.
 *
 * A family that holds a model writes nothing to the serial log, so a test
 * or a benchmark has no other way to say how it did. A write here is one
 * line of `name=value` pairs, each name from the list below and each value
 * a decimal integer. The line is printed rebuilt from the parsed numbers,
 * so no byte of the guest's own reaches the log; a write with anything
 * else in it is EINVAL and prints nothing.
 */

use alloc::string::String;
use core::fmt::Write;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::dev_metrics_parse::{parse, MAX_LINE};

pub fn report(guest: &mut Guest, buf: u64, len: u64) -> u64 {
    if len == 0 || len > MAX_LINE {
        return errno::fail(errno::EINVAL);
    }
    let Some(bytes) = guest.read(buf, len as usize) else {
        return errno::fail(errno::EFAULT);
    };
    let mut line = String::from("[LINUX] metrics:");
    for pair in bytes.split(|b| b.is_ascii_whitespace()).filter(|p| !p.is_empty()) {
        let Some((name, value)) = parse(pair) else {
            return errno::fail(errno::EINVAL);
        };
        let _ = write!(line, " {name}={value}");
    }
    line.push('\n');
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    errno::ok(len)
}
