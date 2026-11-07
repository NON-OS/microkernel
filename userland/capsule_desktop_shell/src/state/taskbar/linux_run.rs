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

//! The tile a Linux run names for itself. The shell awaits a run's window
//! only for a run it queued (run_window.rs), so a Qwen window opened from the
//! Terminal's `qwen window` was never tracked and came back from the dock no
//! more than before, and a store run queued in the same pass of init ahead of
//! the shell's had its window taken for the shell's app. The run now names
//! its package as it starts, before its guest can show a window
//! (server/handlers/linux_window.rs): a Qwen tier awaits its window as Qwen's
//! whoever asked for the run, and any other package ends the wait, so the
//! next run window is matched to the run that opens it.

use super::run_window::expect_run_window;
use super::types::TaskbarState;
use crate::state::apps::LAUNCHER_APPS;

/// The longest package name a run may name; the tier packages are under 20.
pub const LINUX_PACKAGE_MAX: usize = 64;

/// The prefix of every Qwen tier the personality ships (`qwen-<tier>`).
const QWEN_PACKAGE: &[u8] = b"qwen-";
/// The dock's Qwen tile, by its service (state/apps/table.rs).
const QWEN_SERVICE: &[u8] = b"tool.qwen";

/// The launcher app a run of `package` shows its window for: Qwen for a
/// tier package, None for any other program and for a name that is not a
/// package name at all.
fn linux_tile(package: &[u8]) -> Option<usize> {
    if package.is_empty() || package.len() > LINUX_PACKAGE_MAX {
        return None;
    }
    let tier = package.strip_prefix(QWEN_PACKAGE)?;
    let word = |b: &u8| b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'.' || *b == b'-';
    if tier.is_empty() || !tier.iter().all(word) {
        return None;
    }
    LAUNCHER_APPS.iter().position(|a| a.service == QWEN_SERVICE)
}

/// The run now in the role started for `package`: its window is awaited as
/// the tile's the package names, or as no app's. Returns whether it named a
/// tile.
pub fn run_named(state: &mut TaskbarState, package: &[u8]) -> bool {
    match linux_tile(package) {
        Some(index) => {
            expect_run_window(state, index);
            true
        }
        None => {
            state.run_window = None;
            false
        }
    }
}
