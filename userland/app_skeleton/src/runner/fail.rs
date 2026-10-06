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

use nonos_libc::{mk_exit, mk_getpid};

use crate::log_line::{say, Line};

/// End the app with `code`, saying why in the kernel's log first:
/// `[APP-FAIL] <who>: <why>; the app ends (exit <code>)`, read with `log app-fail`.
/// The line is written before the exit, since nothing of the app is left
/// after it to say anything.
pub(super) fn fail(code: i32, who: Who, why: &[u8]) -> ! {
    let line = match who {
        Who::Titled(title) => Line::new(b"APP-FAIL").text(title),
        Who::Pid => Line::new(b"APP-FAIL").text(b"pid ").num(i64::from(mk_getpid())),
    };
    let _ =
        say(&line.text(b": ").text(why).text(b"; the app ends (exit ").num(code.into()).text(b")"));
    mk_exit(code)
}

/// Whom a failure is said of.
pub(super) enum Who {
    /// The app's window title.
    Titled(&'static [u8]),
    /// Not built yet: its pid, which the kernel's `[SPAWN]` line names.
    Pid,
}
