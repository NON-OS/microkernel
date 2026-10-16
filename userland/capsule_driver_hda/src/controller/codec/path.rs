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

//! A route from an output pin back to a DAC, as `search` finds it.

use super::hop::exclusive;
pub use super::search::{find, Miss};
use super::widget::Codec;

/// Pin, then at most four mixers or selectors, then the DAC. Real codecs
/// route a pin to a DAC in one to three hops.
pub const MAX_DEPTH: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Path {
    /// `nodes[0]` is the pin and `nodes[len - 1]` the DAC.
    pub nodes: [u8; MAX_DEPTH],
    /// `sel[i]` is the index in `nodes[i]`'s connection list of `nodes[i + 1]`.
    pub sel: [u8; MAX_DEPTH],
    pub len: u8,
}

impl Path {
    pub fn dac(&self) -> u8 {
        self.nodes[self.len as usize - 1]
    }

    pub fn hops(&self) -> impl Iterator<Item = (u8, u8)> + '_ {
        (0..self.len as usize).map(|i| (self.nodes[i], self.sel[i]))
    }

    /// The input this path needs `nid` to select, if `nid` is on it and
    /// passes one input at a time.
    pub(super) fn selection_of(&self, codec: &Codec, nid: u8) -> Option<u8> {
        let i = self.nodes[..self.len as usize - 1].iter().position(|&n| n == nid)?;
        let w = codec.get(nid)?;
        if exclusive(w) {
            Some(self.sel[i])
        } else {
            None
        }
    }
}
