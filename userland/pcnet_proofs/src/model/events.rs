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

//! What the part saw the driver do, the faults a test can switch on, and
//! what INIT read.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// A software reset, with how many grants the driver had given back.
    Reset {
        grants_back: usize,
    },
    Csr0(u16),
    Bcr(u16, u16),
    Init,
    ApromRead,
}

/// Ways a part can fail the driver, one switch each.
#[derive(Default, Clone, Copy)]
pub struct Faults {
    pub stuck_in_reset: bool,
    pub deaf_to_init: bool,
    pub never_starts: bool,
}

/// What INIT read out of the init block.
#[derive(Default, Clone, Copy, Debug)]
pub struct Rings {
    pub mode: u16,
    pub padr: [u8; 6],
    pub ladrf: [u8; 8],
    pub rdra: u32,
    pub tdra: u32,
    pub rx_len: u16,
    pub tx_len: u16,
    pub rx_index: u16,
    pub tx_index: u16,
}
