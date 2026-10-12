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

//! What a PCCT subspace says, every architecture alike: its shared
//! memory, its registers with their masks, its interrupt and timings.

use super::le::u64_at;
use super::spcr_tail::{gas, Gas};

/// A register and the two masks that go with it (preserve and write, or
/// mask and set), as the subspace gives them. Only ever read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reg {
    pub gas: Gas,
    pub preserve: u64,
    pub write: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Subspace {
    pub kind: u8,
    pub base: u64,
    pub length: u64,
    pub doorbell: Option<Reg>,
    /// The platform interrupt and its flags (bit 0 edge, bit 1 low).
    pub interrupt: Option<(u32, u8)>,
    pub ack: Option<Reg>,
    pub complete: Option<Reg>,
    pub error: Option<Reg>,
    pub latency_us: u32,
    pub max_rate: u32,
    pub min_turnaround_us: u32,
}

pub(super) fn reg(s: &[u8], at: usize) -> Option<Reg> {
    Some(Reg { gas: gas(s, at)?, preserve: u64_at(s, at + 12)?, write: u64_at(s, at + 20)? })
}
