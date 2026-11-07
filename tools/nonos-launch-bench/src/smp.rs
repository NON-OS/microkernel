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

//! The kernel hashes a long image a serve unit at a time, answering TLB
//! shootdowns between pieces; one hasher fed in order gives the same digest.
//! The host has no shootdowns to answer, so the whole buffer is one piece.

pub fn in_serve_units(data: &[u8], mut step: impl FnMut(&[u8])) {
    step(data);
}
