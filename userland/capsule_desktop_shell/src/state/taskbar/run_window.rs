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

//! The window of a Linux run the shell queued. Qwen's window is opened by
//! the Linux personality, as app.linux.run, not by a process of a launcher
//! app, so the registry names no app for it and the dock never tracked it:
//! once minimised, a click on Qwen's tile asked for a second run, which the
//! kernel refused (the run role is taken), and the window stayed hidden.
//!
//! The shell now remembers which app a run's window is awaited for, and
//! takes the next window opened by the pid the kernel registered as
//! app.linux.run, a name no other capsule may claim, as that app's. The
//! wait is set when the shell queues the run, and again when the run names
//! its package as it starts (linux_run.rs), which also ends it for a run of
//! any other package, so a store run that took the role first is never
//! taken for the shell's. Any later window of the same process (a popup) is
//! left untracked.

use super::types::TaskbarState;

/// App `index` asked the kernel for its run's window, which was queued.
pub fn expect_run_window(state: &mut TaskbarState, index: usize) {
    if index < state.open.len() {
        state.run_window = Some(index as u8);
    }
}

/// The app a window opened by `pid` belongs to, when it is the awaited run
/// window: `run_pid` says the pid registered as app.linux.run now, and is
/// asked only while a run window is awaited. The wait ends with the window
/// it names.
pub fn run_window_app(
    state: &mut TaskbarState,
    pid: u32,
    run_pid: impl FnOnce() -> Option<u32>,
) -> Option<usize> {
    state.run_window?;
    if pid == 0 || run_pid() != Some(pid) {
        return None;
    }
    state.run_window.take().map(usize::from)
}
