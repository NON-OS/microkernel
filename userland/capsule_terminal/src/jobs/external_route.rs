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

use super::box_hold::BoxHold;
use super::capture::Capture;
use crate::command::output::Output;

/// One message from the program: a boxed run's report frame is held for the
/// end, never shown or captured; anything else goes where its output goes.
pub(super) fn route(
    msg: &[u8],
    capture: Option<&mut Capture>,
    boxed: Option<&mut BoxHold>,
    out: &mut Output<'_>,
) {
    if boxed.is_some_and(|hold| hold.take(msg)) {
        return;
    }
    match capture {
        Some(capture) => capture.route(msg, out),
        None => out.feed_raw(msg),
    }
}
