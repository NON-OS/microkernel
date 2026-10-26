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

//! The line an unboxed run ends with when its policy refused it anything:
//! how many refusals, kind by kind, and nothing that names a process, so
//! two runs of one program leave the same line for the same refusals.

use alloc::string::String;
use core::fmt::Write;

use super::kind::ALL;
use super::tally::Tally;

/// `None` for a run nothing was refused; the kinds with no refusal are left out.
pub fn run_line(tally: &Tally) -> Option<String> {
    if tally.total() == 0 {
        return None;
    }
    let mut s = String::new();
    let _ = write!(s, "[LINUX] its policy refused {}:", tally.total());
    for kind in ALL.iter().filter(|k| tally.count(**k) != 0) {
        let _ = write!(s, " {}={}", kind.tag(), tally.count(*kind));
    }
    s.push('\n');
    Some(s)
}
