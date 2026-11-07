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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RingError {
    /// The slot storage is not `capacity × W` words, or the capacity is
    /// not a power of two between 2 and 2^30.
    Shape,
    /// No free slot: the consumer has not released enough.
    Full,
    /// The other side's index is further away than the ring holds: the
    /// peer wrote nonsense into the header. Nothing was read or written.
    Corrupt,
}
