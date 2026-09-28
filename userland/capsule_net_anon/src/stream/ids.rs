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

//! Allocating a stream id inside a circuit.

/*
 * Stream id zero is reserved for messages about the circuit itself, so an
 * allocator must never hand it out. Ids are per circuit, so two circuits may
 * both carry stream 1 without ambiguity.
 */
/// The next id after `previous`, skipping zero on wrap.
pub fn next(previous: u16) -> u16 {
    match previous.checked_add(1) {
        Some(0) | None => 1,
        Some(id) => id,
    }
}
