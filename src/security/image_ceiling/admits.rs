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

use super::rule::over;
use super::value::ceiling;

pub const fn excess(requested: u64) -> u64 {
    over(requested, ceiling())
}

pub const fn admits(requested: u64) -> bool {
    excess(requested) == 0
}

fn name_the_excess(l: &mut crate::sys::serial::Line, requested: u64) {
    l.str(b": image forbids");
    for cap in crate::capabilities::Capability::all() {
        if excess(requested) & cap.bit() != 0 {
            l.str(b" ").str(cap.as_str().as_bytes());
        }
    }
    l.end();
}

/// Names the bits, because a capsule refused here was correctly signed and
/// that has to be distinguishable from a broken signature.
pub fn report(name: &str, requested: u64) {
    let mut l = crate::sys::serial::Line::new();
    name_the_excess(l.str(b"[CEILING] refused ").str(name.as_bytes()), requested);
}

/// The mint path has a pid and no name.
pub fn report_pid(pid: u32, requested: u64) {
    let mut l = crate::sys::serial::Line::new();
    name_the_excess(l.str(b"[CEILING] refused pid ").dec(pid as u64), requested);
}
