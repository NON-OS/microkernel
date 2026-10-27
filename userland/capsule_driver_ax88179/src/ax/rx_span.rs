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

//! What parsing one receive transfer gives (rx_header.rs): the frames in
//! it, and what it held that was not taken.

/// A frame in the transfer: where it starts and its length.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Span {
    pub at: u16,
    pub len: u16,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Tally {
    /// The trailer or table lies outside the transfer: nothing taken.
    pub lie: bool,
    /// Frames the chip marked bad (CRC error, dropped) or under a header.
    pub errors: u32,
    /// Frames longer than a full frame and its FCS.
    pub too_long: u32,
    /// An entry named bytes past the frames: it and the rest let go.
    pub beyond: bool,
    /// Frames past the most one transfer may give.
    pub over: u32,
}
