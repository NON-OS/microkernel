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

//! One address with money due (v1.4.56): why, since when, and its last read.

use super::landed_held::Held;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Why {
    RefundSent,
    DisputeSent,
    RefundOwed,
    /// A CLOSE this wallet sent: the channel's reclaimTo (v1.4.60).
    ChannelClosed,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Entry {
    pub address: [u8; 20],
    pub why: Why,
    pub since_ms: u64,
    pub read_ms: Option<u64>,
    /// NOX and ETH the last read found, if both answered.
    pub held: Option<Held>,
}
