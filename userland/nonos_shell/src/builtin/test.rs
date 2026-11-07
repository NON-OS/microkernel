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

//! `test`: a condition for `if` and `while`, by its status alone.
//! `test A = B` and `test A != B` compare text; `-eq -ne -lt -le -gt -ge`
//! compare whole numbers; `test -n S` and `test -z S` ask whether S is
//! empty; `test S` is true for a non-empty S. A malformed test prints its
//! usage and returns 2, as the POSIX utility does.

use crate::out::ShellOut;

/// The status: 0 true, 1 false, 2 malformed.
pub fn run(out: &mut dyn ShellOut, argv: &[&[u8]]) -> i32 {
    let truth = match argv.get(1..).unwrap_or(&[]) {
        [] => Some(false),
        [s] => Some(!s.is_empty()),
        [b"-n", s] => Some(!s.is_empty()),
        [b"-z", s] => Some(s.is_empty()),
        [a, b"=", b] => Some(a == b),
        [a, b"!=", b] => Some(a != b),
        [a, op, b] => numbers(a, op, b),
        _ => None,
    };
    match truth {
        Some(t) => i32::from(!t),
        None => {
            out.writeln(b"usage: test A = B | A != B | N -eq|-ne|-lt|-le|-gt|-ge M | -n S | -z S");
            2
        }
    }
}

fn numbers(a: &[u8], op: &[u8], b: &[u8]) -> Option<bool> {
    let (a, b) = (int(a)?, int(b)?);
    Some(match op {
        b"-eq" => a == b,
        b"-ne" => a != b,
        b"-lt" => a < b,
        b"-le" => a <= b,
        b"-gt" => a > b,
        b"-ge" => a >= b,
        _ => return None,
    })
}

fn int(s: &[u8]) -> Option<i64> {
    core::str::from_utf8(s).ok()?.parse().ok()
}
