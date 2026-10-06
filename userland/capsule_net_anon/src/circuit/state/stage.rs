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

//! How far a circuit has got.

/// Named rather than a hop count: two hops up with a handshake in flight is a
/// different state from two hops up, and only one of them may be extended.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CircuitStage {
    Handshaking,
    Open,
    Dead,
}
