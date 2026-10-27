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

//! What the chip sent that was not taken whole, counted: a proof, or a
//! reader of the bound driver, can see each was let go and never used.

use super::rx_span::Tally;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Counts {
    /// Transfers whose trailer or table lies outside the transfer: dropped
    /// whole.
    pub lies: u32,
    /// Frames the chip marked bad (CRC, dropped) or shorter than a header.
    pub errors: u32,
    /// Frames longer than a frame and its FCS, or than the stack's room.
    pub too_long: u32,
    /// Table entries naming bytes past the frames; the rest of the table.
    pub beyond: u32,
    /// Frames past SPANS_MAX in one transfer.
    pub over: u32,
    /// Transfers that filled the receive buffer without ending.
    pub overruns: u32,
    /// Link reports short or malformed.
    pub notices: u32,
}

impl Counts {
    pub(super) fn add(&mut self, t: Tally) {
        bump(&mut self.lies, t.lie as u32);
        bump(&mut self.errors, t.errors);
        bump(&mut self.too_long, t.too_long);
        bump(&mut self.beyond, t.beyond as u32);
        bump(&mut self.over, t.over);
    }
}

pub(super) fn bump(n: &mut u32, by: u32) {
    *n = n.saturating_add(by);
}
