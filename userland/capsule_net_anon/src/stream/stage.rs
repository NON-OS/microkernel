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

//! How far a stream has got.

/// How far a stream has got.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StreamStage {
    Opening,
    Open,
    Ended(u8),
}

impl StreamStage {
    /// Whether the far end may still send on this stream.
    ///
    pub fn is_live(self) -> bool {
        matches!(self, StreamStage::Opening | StreamStage::Open)
    }
}
