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

//! What a running app's window holds between frames.

use nonos_toolkit::decorations::FrameState;

use crate::app::{App, AppManifest};
use crate::setup::WindowBinding;

use super::drag::DragState;
use super::held::Held;

pub(super) struct BootedApp<A: App> {
    pub app: A,
    pub manifest: AppManifest,
    pub binding: WindowBinding,
    pub input_ready: bool,
    /// Uptime of the last input subscription attempt, for the heartbeat.
    pub input_beat_ms: i64,
    /// A message a paced wait received that no drain has handled yet.
    pub held: Option<Held>,
    pub primed: bool,
    pub maximized: bool,
    pub minimized: bool,
    pub saved: (u32, u32, u32, u32),
    /// What the app asks of its window (full screen, a rect), and what it took.
    pub asks: super::asks::Asks,
    pub drag: DragState,
    /// Whether the window has the keyboard, by the WM's word (inbox/keys.rs).
    /// A window opens with it.
    pub focused: bool,
    /* Size, frame state and maximized state of the last repaint, so a repaint
     * that changes none of them can leave the frame as it is. */
    pub painted: (u32, u32, FrameState, bool),
}
