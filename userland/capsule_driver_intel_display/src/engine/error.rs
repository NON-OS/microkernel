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

/// Why the engine refused or gave up. None of these leave the display
/// changed: every check runs before the first write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// No pipe is scanning out: firmware left nothing to take over.
    NothingLit,
    /// The firmware's plane is in a layout this driver does not flip
    /// (tiled, compressed, rotated, or not 32-bit RGB).
    Layout,
    /// A size, stride or address the engine cannot take.
    Argument,
    /// The GGTT has no free run of the size asked for.
    GgttFull,
    /// The hardware did not latch a change before the deadline.
    Timeout,
}
