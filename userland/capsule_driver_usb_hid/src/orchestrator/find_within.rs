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

//! A bounded search: pure, so the bound is proven on the host.

/// Up to `attempts` tries of `find`, with a `pause` between two of them and
/// none after the last, so the wait is bounded by the tries and the pauses.
pub fn find_within<T>(
    attempts: u32,
    mut find: impl FnMut() -> Option<T>,
    mut pause: impl FnMut(),
) -> Option<T> {
    for attempt in 0..attempts {
        if attempt > 0 {
            pause();
        }
        if let Some(found) = find() {
            return Some(found);
        }
    }
    None
}
