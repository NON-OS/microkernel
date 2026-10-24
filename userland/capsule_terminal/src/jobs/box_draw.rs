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

//! A boxed run's report drawn once the run has ended, after whatever the
//! guest left half-written is abandoned (CAN), so no escape it began can
//! swallow the report; and its `[BOX]` line sent to the serial log.

use super::box_hold::BoxHold;
use crate::command::output::Output;

const _: () = assert!(super::box_hold::TAG == nonos_libc::TAG_BOX);

pub(super) fn draw(hold: &BoxHold, out: &mut Output<'_>) {
    out.feed_raw(b"\x18");
    let (shown, line) = hold.lines();
    if shown.is_empty() {
        out.writeln_error(b"linux: the box sent no report; it may have been stopped");
    }
    for l in shown {
        out.writeln(l);
    }
    if let Some(l) = line {
        let mut l = l.to_vec();
        l.push(b'\n');
        let _ = nonos_libc::mk_debug(l.as_ptr(), l.len());
    }
}

/// The run was stopped before its report could come.
pub(super) fn interrupted(out: &mut Output<'_>) {
    out.writeln_error(b"linux: stopped before the box's report arrived");
}
