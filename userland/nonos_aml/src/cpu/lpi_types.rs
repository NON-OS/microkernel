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

//! What one _LPI level holds once read; `lpi` says how it is read.

use super::register::Register;
use alloc::vec::Vec;

/// How a state is entered: a register to write, or an integer that is
/// combined with the levels below it into the one value written for all.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LpiEntry {
    Register(Register),
    Integer(u64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LpiState {
    pub min_residency_us: u32,
    pub wake_latency_us: u32,
    pub flags: u32,
    /// Which architectural context the state loses; each architecture
    /// defines the bits.
    pub context_lost: u64,
    /// The residency counter's rate; 0 means the architecture's own.
    pub counter_hz: u64,
    /// The parent level's state this one allows, counted from 1; 0 for
    /// none.
    pub parent_state: u32,
    pub entry: LpiEntry,
    pub residency: Option<Register>,
    pub usage: Option<Register>,
    /// For the log only; at most `MAX_NAME` bytes kept.
    pub name: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lpi {
    pub revision: u64,
    pub level: u64,
    pub states: Vec<LpiState>,
}
