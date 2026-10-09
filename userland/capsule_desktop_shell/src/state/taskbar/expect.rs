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

//! A window the shell asked for and is waiting to see. A click that queued a
//! spawn, or woke an app's idle process to open, said "opening a new window"
//! and nothing more: when init refused the spawn (it says why on serial
//! only), or the process never opened its window, the person saw a dock that
//! did nothing. Each such click is now followed until the window manager
//! says a window of that app opened, and past the deadline the shell says
//! the app did not open.

use super::types::{TaskbarState, Uptime};

/// How long a window may take: an attested spawn queued behind init's other
/// work, its boot frame, and the window's first open and paint. Sized for a
/// slow laptop (a Celeron N4120): each spawn checks a proof of about 115 KB
/// and loads the app's ELF in init, behind whatever init is doing, and a
/// large app (Browser) builds its engine before it asks for a window. 15 s
/// said "did not open" there for a window that then came. 30 s is the
/// installed apps' launch deadline (state/launch.rs), which waits on the
/// same attested load; a launch that truly failed is said that much later,
/// and its reason is in `log launch`, `log spawn-instance` and `log app-fail`.
pub const EXPECT_MS: i64 = 30_000;

/// Wait for a window of app `index`, asked for at `now`. A second ask for
/// the same app restarts its wait.
pub fn expect_window(state: &mut TaskbarState, index: usize, now: Uptime) {
    if index >= state.open.len() {
        return;
    }
    state.expecting.retain(|e| e.0 as usize != index);
    state.expecting.push((index as u8, now.0));
}

/// A window of app `index` opened: its wait is over.
pub fn window_came(state: &mut TaskbarState, index: usize) {
    state.expecting.retain(|e| e.0 as usize != index);
}

/// The apps whose window had not come by `now`, as a mask by launcher
/// index; their waits end here.
pub fn windows_overdue(state: &mut TaskbarState, now: Uptime) -> u64 {
    let mut late = 0u64;
    state.expecting.retain(|&(app, at)| {
        if now.0.saturating_sub(at) < EXPECT_MS {
            return true;
        }
        late |= 1u64.checked_shl(u32::from(app)).unwrap_or(0);
        false
    });
    late
}
