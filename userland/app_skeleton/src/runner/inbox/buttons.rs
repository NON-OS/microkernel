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
//! A press on the frame's own buttons: close, minimise and full screen.

use crate::app::{App, EventOutcome};
use crate::input::InputEvent;

use super::super::decorations;
use super::result::{DrainResult, Frame};

/// Some(true) when `event` pressed a button that ends the drain, Some(false)
/// when it pressed close on an app that keeps its window, None otherwise.
pub(super) fn buttons<A: App>(
    app: &mut A,
    out: &mut DrainResult,
    f: Frame,
    event: InputEvent,
) -> Option<bool> {
    match decorations::handle(f.width, f.height, f.maximized, event)? {
        // An app with work the user would lose keeps its window and says
        // why; the next press on close is the answer.
        EventOutcome::Close if !app.close_requested() => {
            out.repaint = true;
            Some(false)
        }
        EventOutcome::Close => Some(stop(&mut out.close)),
        EventOutcome::Minimize => Some(stop(&mut out.minimize)),
        EventOutcome::Maximize => Some(stop(&mut out.maximize)),
        _ => None,
    }
}

/// Raise `flag` and end the drain.
pub(super) fn stop(flag: &mut bool) -> bool {
    *flag = true;
    true
}
