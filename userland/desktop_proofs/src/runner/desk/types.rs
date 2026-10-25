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

//! A window's client side, and the desk that holds every part of the path.

use super::super::drag::DragState;
use crate::damage::{DamageAccumulator, Rect as Damage};
use crate::focus::FocusModel;
use crate::press::Press;
use crate::scene::SceneTable;
use crate::window::WindowTable;
use crate::z_order::ZStack;

/// One window's client side: where it believes it is, and its frame state.
pub struct App {
    pub pid: u32,
    pub window_id: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub accessory_w: u32,
    pub drag: DragState,
    pub accessory_events: u32,
    pub moves: u32,
    /// The size the window opened at, which min_size holds it to.
    pub opened: (u32, u32),
    /// The rect green restores, while the window is full screen.
    pub saved: Option<(u32, u32, u32, u32)>,
}

pub struct Desk {
    pub width: u32,
    pub height: u32,
    pub windows: WindowTable,
    pub z: ZStack,
    pub focus: FocusModel,
    pub scene: SceneTable,
    pub damage: DamageAccumulator,
    pub screen: Vec<u32>,
    pub press: Option<Press>,
    pub apps: Vec<App>,
    pub shell_presses: u32,
    /// The desktop shell's pid once it is up, else 0. A press on no window
    /// goes to it.
    pub shell: u32,
    /// The compositor takes no focus_set: each one the window manager sends
    /// is lost (the window manager owes it a restack, state/restack.rs).
    pub compositor_lost: bool,
    /// The shell's chrome surface, a full-screen layer over every window:
    /// what it holds at each pixel (CLEAR, PANEL, SHADOW). The compositor
    /// shows the window under it where it is clear.
    pub chrome: Vec<u8>,
    /// The shell's dock: its rule (state/taskbar, the real code) and rect.
    pub taskbar: crate::taskbar::TaskbarState,
    pub dock: (u32, u32, u32, u32),
    /// Rects the shell committed as damage for its dock.
    pub dock_commits: Vec<Damage>,
}
