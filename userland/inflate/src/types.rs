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

//! What a partial decode hands back.

use alloc::vec::Vec;

/// How a decode ended.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum End {
    /// The stream ended where its format says, and its checks passed.
    Complete,
    /// The input ran out first; the output holds all the input decoded to.
    Truncated,
    /// The output reached the caller's cap; the rest was not decoded.
    Capped,
    /// The input stops being a valid stream at some point.
    Corrupt,
}

/// The output so far, how the decode ended, and the input bytes it read.
pub struct Inflated {
    pub out: Vec<u8>,
    pub end: End,
    pub used: usize,
}

impl Inflated {
    /// The output when the stream decoded completely, else nothing.
    pub fn complete(self) -> Option<Vec<u8>> {
        (self.end == End::Complete).then_some(self.out)
    }
}
