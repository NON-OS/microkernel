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

//! The host's registers and record, and the device's own state.

/// What the host has seen and done, for the tests to hold it to.
#[derive(Default)]
pub struct Host {
    /// One word per register, by offset / 4, up to UCMDARG3 at 0x9C.
    pub regs: Vec<u32>,
    /// Reads of HCE that still answer 0 after it was set.
    pub hce_reads: u32,
    /// Times HCE was set to 1.
    pub enables: u32,
    pub link_tries: u32,
    pub lcc: u32,
    /// Every UIC command, with its three arguments.
    pub uic: Vec<(u32, [u32; 3])>,
    /// Every UTRD's command type and data direction.
    pub utrds: Vec<(u32, u32)>,
    /// UTRLCLR writes.
    pub clears: Vec<u32>,
}

/// The device's own state beside its logical units.
#[derive(Default)]
pub struct Dev {
    pub nops: u32,
    pub init_flag: bool,
    pub init_left: u32,
    /// Queries answered so far, and how many of the first are refused
    /// (INVALID_OPCODE) whatever they ask.
    pub queries: u32,
    pub refuse_queries: u32,
}
