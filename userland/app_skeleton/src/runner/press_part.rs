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

//! Which part of a window a pointer event belongs to while a button is held.
//!
//! A press picks the part under it: the widget an app hosts in its titlebar
//! (the terminal's tabs, the settings search field), or the rest of the window
//! (the frame that drags and resizes it, and the content). Every pointer event
//! up to and including the release then goes to that same part, wherever the
//! pointer is by then, as the input router already does between windows.
//!
//! Events used to go to whatever part was under the pointer at each step. A
//! titlebar drag runs in the coordinates of the window as it was at the press,
//! so dragging a terminal to the right carried the pointer, in those
//! coordinates, across where its tabs had been: those steps went to the tabs
//! and the window stopped following, and a release there never reached the
//! drag, which then went on moving the window after the button was up.

use nonos_toolkit::decorations::Rect;

use crate::input::{InputEvent, InputKind};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Part {
    Accessory,
    Window,
}

/// Where an event goes: to the titlebar widget, in its own coordinates, or on
/// to the frame and the content, unchanged.
pub(super) enum Route {
    Accessory(InputEvent),
    Window(InputEvent),
}

#[derive(Clone, Copy)]
pub(super) struct PressGrab {
    held: Option<Part>,
}

impl PressGrab {
    pub(super) const fn new() -> Self {
        Self { held: None }
    }

    /// Route `event`. `accessory` is the titlebar widget's rectangle in window
    /// coordinates, None when the app hosts none. A press that began on the
    /// widget and is dragged off it keeps reaching it, with coordinates that
    /// run negative or past its size; widgets act on a press inside their own
    /// bounds only, so that is safe.
    pub(super) fn route(&mut self, event: InputEvent, accessory: Option<Rect>) -> Route {
        if !event.is_pointer() {
            return Route::Window(event);
        }
        let over = match accessory {
            Some(a)
                if event.x >= 0 && event.y >= 0 && a.contains(event.x as u32, event.y as u32) =>
            {
                Part::Accessory
            }
            _ => Part::Window,
        };
        let part = match event.kind {
            InputKind::ButtonDown => {
                self.held = Some(over);
                over
            }
            InputKind::ButtonUp => self.held.take().unwrap_or(over),
            _ => self.held.unwrap_or(over),
        };
        match (part, accessory) {
            (Part::Accessory, Some(a)) => {
                let mut local = event;
                local.x = local.x.saturating_sub(a.x as i32);
                local.y = local.y.saturating_sub(a.y as i32);
                Route::Accessory(local)
            }
            _ => Route::Window(event),
        }
    }
}
