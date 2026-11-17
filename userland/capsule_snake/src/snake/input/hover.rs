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
//! Feeds the pointer and names the control under it, by the same test a click
//! makes, so a control draws hover and press and a move repaints only on
//! crossing to another control.

use nonos_app_skeleton::{EventOutcome, InputEvent, InputKind};
use nonos_toolkit::widgets::{Pointer, State as Look};

use crate::snake::state::Screen;

use super::target::{self, Target};

/// The pointer over the window and the control it names, if any.
#[derive(Clone, Copy, Default)]
pub struct Hover {
    pub pointer: Pointer,
    pub target: Option<Target>,
}

impl Hover {
    /// A pointer not seen yet, over nothing.
    pub const fn new() -> Self {
        Hover { pointer: Pointer::new(), target: None }
    }

    /// Moves and releases, neither of which clicks; None for any other event.
    /// The input router says the pointer left with a move to (-1, -1), which
    /// lands left of and above the content. A release repaints what may have
    /// been held.
    pub fn on_pointer(&mut self, screen: Screen, ev: &InputEvent) -> Option<EventOutcome> {
        match ev.kind {
            InputKind::PointerAbs if ev.x < 0 && ev.y < 0 => self.pointer.left(),
            InputKind::PointerAbs => self.pointer.moved(ev.x, ev.y),
            InputKind::ButtonUp => self.pointer.released(),
            _ => return None,
        }
        let held = ev.kind == InputKind::ButtonUp && self.target.is_some();
        let moved = self.rehover(screen);
        Some(if held || moved { EventOutcome::Repaint } else { EventOutcome::Idle })
    }

    /// Names the control under the pointer on `screen` again; true when that
    /// changed.
    pub fn rehover(&mut self, screen: Screen) -> bool {
        let next = self.pointer.position().and_then(|(x, y)| target::at(screen, x, y));
        core::mem::replace(&mut self.target, next) != next
    }

    /// The hover and press the control `t` on `screen` draws. Only the control
    /// the hover names asks the pointer, so a paint tests one control again.
    pub fn look(&self, screen: Screen, t: Target) -> Look {
        if self.target != Some(t) {
            return Look::default();
        }
        self.pointer.state(|x, y| target::at(screen, x, y) == Some(t))
    }
}
