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

//! A boxed run's report drawn once the run has ended, on a screen put back
//! to its defaults first (box_screen::SCREEN_RESET), so no mode the guest left
//! can hide or bend it; and its `[BOX]` line, after its `[BOX-NET]` line when
//! it names destinations, sent to the serial log.

use super::box_hold::BoxHold;
use super::box_missing::{line as missing_line, said, Missing};
use super::box_screen::SCREEN_RESET;
use crate::command::output::Output;

const _: () = assert!(super::box_hold::TAG == nonos_libc::TAG_BOX);

pub(super) fn draw(hold: &BoxHold, out: &mut Output<'_>) {
    out.feed_raw(SCREEN_RESET);
    let (shown, line) = hold.lines();
    let exit = hold.ended.unwrap_or(0);
    if shown.is_empty() {
        missing(Missing::None, exit, out);
    }
    for l in shown {
        out.writeln(l);
    }
    match line {
        Some(l) => {
            if !hold.net_lines().into_iter().chain([l]).all(to_log) {
                out.writeln_error(b"linux: the [BOX] line could not be written to the log");
            }
        }
        None if !hold.lines().0.is_empty() => missing(Missing::Incomplete, exit, out),
        None => {}
    }
}

/// A report that never came whole, said on the screen and to the serial log.
pub(super) fn missing(m: Missing, exit: i32, out: &mut Output<'_>) {
    out.writeln_error(said(m, exit));
    let l = missing_line(m, exit);
    if !to_log(l.as_bytes()) {
        out.writeln_error(b"linux: the [BOX-MISSING] line could not be written to the log");
    }
}

/// The one way a line reaches the serial log from here: the report's own
/// `[BOX-NET]` and `[BOX]` lines, from its keyed frames, or this module's
/// `[BOX-MISSING]`.
fn to_log(line: &[u8]) -> bool {
    super::box_log::log(line, |p| nonos_libc::mk_debug(p.as_ptr(), p.len()))
}
