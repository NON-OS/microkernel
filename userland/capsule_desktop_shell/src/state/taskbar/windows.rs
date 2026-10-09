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

//! The app windows the window manager has said are open, by owner and window
//! id: an app's window id is fixed in its manifest, so every instance of an
//! app opens a window with the same id, and only the pid tells them apart. An app
//! can have several (one per numbered instance), so the dock's running mark
//! is "any window of this app is open", not the last event seen: closing a
//! second terminal used to put the mark out while the first was still up.
//! The app a closing window belonged to is remembered from its open, since
//! the closing process may already be gone from the service registry.

use super::set_open::set_taskbar_open;
use super::types::{TaskbarState, TrackedWindow};

/// Cap on remembered windows; the kernel's instance tables allow far fewer.
const TRACKED_MAX: usize = 64;

/// Record that `window_id`, owned by `pid`, of app `index` opened.
pub fn track_window_opened(state: &mut TaskbarState, window_id: u32, pid: u32, index: usize) {
    if index >= state.open.len() {
        return;
    }
    state.windows.retain(|w| !(w.window_id == window_id && w.pid == pid));
    if state.windows.len() >= TRACKED_MAX {
        state.windows.remove(0);
    }
    state.windows.push(TrackedWindow { window_id, pid, app: index as u8 });
    set_taskbar_open(state, index, true);
    super::expect::window_came(state, index);
}

/// Record that `pid`'s window `window_id` closed. Returns the app it belonged to, when the
/// shell saw it open. The app stays marked running while another of its
/// windows is open.
pub fn track_window_closed(state: &mut TaskbarState, pid: u32, window_id: u32) -> Option<usize> {
    let at = state.windows.iter().position(|w| w.window_id == window_id && w.pid == pid)?;
    let gone = state.windows.remove(at);
    let index = gone.app as usize;
    if !app_has_window(state, index) {
        set_taskbar_open(state, index, false);
    }
    Some(index)
}

/// Forget every window whose process `alive` says has ended, as if its close
/// had come: the running mark, the menubar's app and the dock follow, and a
/// full-screen window of that process no longer hides the dock
/// (dock_rule.rs). Returns the apps that lost a window, as a mask by
/// launcher index; the dock's own change is in `visible`.
///
/// Only the window manager's close event cleared a window here. When that
/// event never came (its process left without one, or the event was lost),
/// the app stayed marked running and named in the menubar for the rest of
/// the session.
pub fn forget_dead_windows(state: &mut TaskbarState, alive: impl Fn(u32) -> bool) -> u64 {
    let mut lost = 0u64;
    super::dock_rule::forget_dead_full_screen(state, &alive);
    while let Some(w) = state.windows.iter().find(|w| !alive(w.pid)).copied() {
        if let Some(index) = track_window_closed(state, w.pid, w.window_id) {
            lost |= 1u64.checked_shl(index as u32).unwrap_or(0);
        }
    }
    lost
}

/// Whether any window of app `index` is still open.
fn app_has_window(state: &TaskbarState, index: usize) -> bool {
    state.windows.iter().any(|w| w.app as usize == index)
}
