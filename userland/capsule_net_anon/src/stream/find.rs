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

//! Finding a stream by the circuit that carries it as well as its id.

use super::table::Stream;

/// The stream `id` on circuit `circuit`, and no other.
///
/// Stream ids are only unique within a circuit, and a relay cell names the
/// stream by id alone. Finding it by id alone let a cell arriving on one
/// circuit write into, end or credit a stream carried by another: a hostile
/// exit could inject payload into a connection it never carried.
pub fn find_on(streams: &mut [Stream], circuit: u32, id: u16) -> Option<&mut Stream> {
    streams.iter_mut().find(|s| s.circuit == circuit && s.id == id)
}
