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

//! When a held run must go to the device before the next sector joins it,
//! pure: kernel_proofs::pending_run.

/// The run holds `held` sectors from device LBA `start`. A sector for `at`
/// joins it only when it follows the last one and the run has fewer than
/// `cap`; a run whose end would pass the last LBA never takes another.
pub(super) const fn must_send(start: u64, held: u64, at: u64, cap: u64) -> bool {
    if held == 0 {
        return false;
    }
    let follows = match start.checked_add(held) {
        Some(next) => next == at,
        None => false,
    };
    !follows || held >= cap
}
