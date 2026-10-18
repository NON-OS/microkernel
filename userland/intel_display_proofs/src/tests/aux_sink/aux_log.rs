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

//! What a DisplayPort sink's lies are drawn at, and what it logs of each
//! I2C transaction.

use super::sink::Fault;

/// Percent of transfers answered other than cleanly; `odd`: one in this
/// many ACKs short, and as many long.
#[derive(Clone, Copy, Debug, Default)]
pub struct Rates {
    pub timeout: u32,
    pub defer: u32,
    pub nack: u32,
    pub undefined: u32,
    pub empty: u32,
    pub odd: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Try {
    /// The segment written, and whether it was acknowledged.
    pub segment: Option<(u8, bool)>,
    pub offset: Option<u8>,
    /// Bytes of ACKed reads, as many as each asked at most.
    pub got: Vec<u8>,
    pub read_unaimed: bool,
    /// A stop was answered with an ACK.
    pub stop_ok: bool,
    pub fault: Fault,
    pub t0: u64,
    pub t1: u64,
}
