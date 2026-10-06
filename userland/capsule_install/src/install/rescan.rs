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

/*
 * When the disk list is looked at again on its own. A driver can come up
 * after the list was first made, on a slow controller or after a retry,
 * so a list with nothing to install to, or with a row for a driver that
 * is missing, is looked at again every two seconds while it is on screen.
 * A look waits on every driver that is registered, and one that does not
 * answer holds it for its whole timeout; a look that took long is spaced
 * out by four times what it took, so the screen keeps taking keys.
 */

pub const EVERY_MS: i64 = 2000;

/// Whether a look is due at `now`, the last having ended at `ended` after
/// taking `took`. All in milliseconds of uptime.
pub fn due(now: i64, ended: i64, took: i64) -> bool {
    now.saturating_sub(ended) >= EVERY_MS.max(took.max(0).saturating_mul(4))
}
