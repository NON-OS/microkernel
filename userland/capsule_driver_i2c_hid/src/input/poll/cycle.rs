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
use super::multitouch::multitouch;
use super::pointer::{act, note, pointer};
use super::read_frame::read_frame;
use super::repeat::skip_repeat;
use super::report::{report_of, Report};
use crate::state::{LastRead, State};

/// Read the pad once and post what its report says. True when a fresh
/// report came: not the zero-length one, and not a verbatim repeat.
pub fn poll(state: &mut State) -> bool {
    if !state.found() || state.input_register == 0 {
        return false;
    }
    // A full Precision Touchpad report carries several contacts plus the
    // contact-count and scan-time trailer and can run past 64 bytes; the
    // length prefix at the head still drives how much of this we parse.
    let mut frame = core::mem::take(&mut state.frame);
    let fresh = read_frame(state, &mut frame).is_some_and(|n| deliver(state, &frame, n));
    // The report has been handed on to the input ring; none of it stays in
    // the driver.
    frame.fill(0);
    state.frame = frame;
    fresh
}

/// A frame of `n` bytes read: its report decoded, unless it is none the
/// driver reads, and acted on, unless it repeats the last.
fn deliver(state: &mut State, frame: &[u8], n: usize) -> bool {
    let report = match (state.last_read, report_of(frame, n)) {
        (LastRead::Report, Report::Whole(report)) => report,
        // A torn frame is skipped whole, never decoded from what arrived.
        (LastRead::Report, Report::Torn) => return ended(state, true),
        _ => return ended(state, false),
    };
    // Any report but the driven collections' (a keyboard's keys, a
    // vendor's data) is dropped unread: nothing of it is kept.
    let seen = match state.mt.serves() {
        true => multitouch(state, report),
        false => pointer(state, report),
    };
    let Some(seen) = seen else { return ended(state, true) };
    note(state, &seen);
    if skip_repeat(&mut state.repeat, seen) {
        return false;
    }
    act(state, seen);
    true
}

/// A read that brought no pointer report ends a run of repeats.
fn ended(state: &mut State, report: bool) -> bool {
    state.repeat.reset();
    report
}
