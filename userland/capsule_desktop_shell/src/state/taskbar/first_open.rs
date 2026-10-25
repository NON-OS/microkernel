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
//! An app's first window goes to the process started for it at boot, which
//! waits idle for an open (app_skeleton runner/idle.rs), before any new
//! instance is spawned. A dock click used to ask the kernel for a new
//! instance first, so the boot Terminal never had a window and the first
//! open spawned "app.terminal.1": two processes for one window. Pure; held
//! in desktop_proofs.

/// How an app with no window open is opened.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FirstOpen {
    /// The boot process took the open frame and opens its window.
    Woken,
    /// There is none, or it took nothing: a new instance is asked for.
    Spawn,
}

/// `base` is the pid the app's own name resolves to; `take` sends it the
/// open frame and says whether it was taken.
pub fn first_open(base: Option<u32>, take: impl FnOnce(u32) -> bool) -> FirstOpen {
    match base {
        Some(pid) if take(pid) => FirstOpen::Woken,
        _ => FirstOpen::Spawn,
    }
}
