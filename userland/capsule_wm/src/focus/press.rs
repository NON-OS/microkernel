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

//! Click to focus, as one step. The input router reports every press that
//! lands on a window; that window takes focus and goes on top of the stack
//! together, before the press itself is delivered to it.
//!
//! The press used to move focus only. The window was drawn on top straight
//! away (the compositor lifts what it is told is focused), but it stayed where
//! it was in this stack until its own client, having received the press, came
//! back with a raise of its own. Until then, and for good with a client that
//! never sends one, the hit test still put the other window on top, and a
//! click on the title bar the user was looking at went to the window behind.

use super::FocusModel;
use crate::window::WindowTable;
use crate::z_order::{raise, ZStack};

/// Why a press could not focus the window it landed on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// No such window: it closed between the hit test and this request.
    NoWindow,
    /// The window takes no focus (a tooltip).
    NotFocusable,
}

/// What the press changed: either of these means the compositor must hear of
/// it, so the window it draws on top is the one the stack has on top.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pressed {
    pub focus_changed: bool,
    pub raised: bool,
}

impl Pressed {
    pub fn restacked(&self) -> bool {
        self.focus_changed || self.raised
    }
}

pub fn press_focus(
    windows: &mut WindowTable,
    z: &mut ZStack,
    focus: &mut FocusModel,
    owner_pid: u32,
    window_id: u32,
) -> Result<Pressed, Refused> {
    let window = windows.find(owner_pid, window_id).ok_or(Refused::NoWindow)?;
    if !window.kind.focusable() {
        return Err(Refused::NotFocusable);
    }
    let raised = raise(windows, z, owner_pid, window_id).ok_or(Refused::NoWindow)?;
    let focus_changed = focus.set(owner_pid, window_id);
    Ok(Pressed { focus_changed, raised })
}
