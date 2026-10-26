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

//! Host proofs for the desktop's pointer path, end to end. A press on a window
//! travels: the input router hit-tests it through the window manager, focuses
//! the window there, arms its press grab and delivers the press in window
//! coordinates; the window manager raises the window and tells the compositor,
//! which restacks its layer; the window's app_skeleton routes the press to its
//! frame, which starts a drag; every motion after that comes back through the
//! press grab, the app moves its layer and its window. Each step is the real
//! source, included under the `crate::` paths its files name each other by.

extern crate alloc;

mod from_wm;
mod pointer_path;
pub mod runner;
mod shell_actions;
mod shell_desk;
mod shell_dock;
mod shell_frames;

pub use from_wm::*;
pub use pointer_path::*;
pub use shell_actions::*;
pub use shell_desk::*;
pub use shell_dock::*;
pub use shell_frames::*;

#[cfg(test)]
mod tests;

// Who the shell believes a key event or a window notice from: only the
// process the registry names for the input router or the window manager.
#[path = "../../capsule_desktop_shell/src/server/peer_rule.rs"]
pub mod shell_peer_rule;

#[cfg(test)]
mod peer_rule_tests;

// The window switcher's order and run, and where its strip sits.
#[path = "../../capsule_desktop_shell/src/state/ctx/switch/mod.rs"]
pub mod shell_switch;
#[path = "../../capsule_desktop_shell/src/render/switch/geometry.rs"]
pub mod switch_geometry;
