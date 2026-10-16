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

//! Where the Wi-Fi panel's selection may stand. It runs over the scanned
//! networks and then the saved ones, so it is kept inside that list.
//!
//! It used to be kept inside the adapter list instead: with one adapter,
//! every return to the Wi-Fi page put the selection back on the first
//! network, whichever one it had been on.

/// The cursor, kept inside `rows` rows (the networks, then the saved ones).
pub fn kept_in(cursor: usize, rows: usize) -> usize {
    cursor.min(rows.saturating_sub(1))
}
