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

//! A run that ends before its guest starts: a boxed one still says so, in
//! its report, so a box that ran nothing is never mistaken for one that
//! refused nothing.

use nonos_libc::mk_exit;

pub(super) fn stop(why: &str, code: i32) -> ! {
    super::boxed::not_started(why, code);
    mk_exit(code)
}

/// A start step's log line as the reason a report gives.
pub(super) fn why(step: &[u8]) -> &str {
    let line = core::str::from_utf8(step).unwrap_or("refused").trim();
    line.strip_prefix("[LINUX] ").unwrap_or(line)
}
