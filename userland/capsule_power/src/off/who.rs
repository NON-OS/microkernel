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

//! Who sent a `power` request, told by the services the sender's process
//! owns now: the shell first, then the two installers, else anyone. The
//! runner asks `runs` of the sender the kernel stamped on the message
//! (runner.rs), never of anything the request carries, and since the
//! kernel lets all three reach this inbox (HELD's `power.off` line), this
//! split is what keeps a power-off the shell's alone.

use super::rule::{Caller, INSTALLERS};

/// The one service whose process may power the machine off.
pub const SHELL: &[u8] = b"desktop_shell";

/// The caller whose process owns a service `runs` names.
pub fn classify(runs: impl Fn(&[u8]) -> bool) -> Caller {
    if runs(SHELL) {
        Caller::Shell
    } else if INSTALLERS.iter().any(|s| runs(s)) {
        Caller::Installer
    } else {
        Caller::Other
    }
}
