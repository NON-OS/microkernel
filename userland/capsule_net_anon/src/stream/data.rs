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

//! Splitting a payload into RELAY_DATA sized pieces.

use crate::cell::RELAY_BODY_BYTES;

/*
 * A relay message body is 498 bytes, so anything longer is several cells. Each
 * costs one from the package window, which is why the caller has to be told how
 * many it is about to spend rather than handing over a buffer and hoping.
 */
/// The pieces of `data`, each fitting one cell.
pub fn pieces(data: &[u8]) -> impl Iterator<Item = &[u8]> {
    data.chunks(RELAY_BODY_BYTES)
}
