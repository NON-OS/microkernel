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

//! Whose process a reset's waits put to sleep (K10a), pure. A wait sleeps
//! only for a caller in its own syscall with no broker lock held: a claim,
//! and MkDeviceRelease. Exit teardown can run from a fault's default signal
//! or from another process's MkKill, so its waits spin.
//! kernel_proofs::function_reset.

/// The process a reset's waits sleep: `named`, only while it is the one
/// running. `None`, and the waits spin, otherwise.
pub(crate) fn sleeper(named: Option<u32>, running: Option<u32>) -> Option<u32> {
    named.filter(|pid| running == Some(*pid))
}
