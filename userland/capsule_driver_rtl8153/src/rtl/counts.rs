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

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Counts {
    /// Transfers too short to hold a descriptor and a packet.
    pub runts: u32,
    /// Descriptors whose length was too short or ran past the transfer;
    /// the rest of that transfer was let go.
    pub broken: u32,
    /// Transfers that filled the receive buffer without ending, let go to
    /// their end.
    pub overruns: u32,
    /// Frames longer than a 1514-byte frame or the room the stack gave.
    pub too_long: u32,
}

pub(super) fn bump(n: &mut u32) {
    *n = n.saturating_add(1);
}
