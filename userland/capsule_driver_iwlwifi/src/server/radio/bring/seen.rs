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

//! What the card said it is, and the radio after a good bring-up.

use alloc::vec::Vec;

use crate::firmware::gen3::dev::Dev;
use crate::regs::Regs;

use super::super::grant::Grant;

/// What the card said it is, for the status reply, and whether this path
/// has touched it (from then on it owns the card).
#[derive(Clone, Copy, Default)]
pub struct Seen {
    pub hw_rev: u32,
    pub rf_id: u32,
    pub touched: bool,
}

/// The radio after a good bring-up.
pub struct Up {
    pub dev: Dev<'static, Regs, Grant>,
    pub channels: Vec<u8>,
    /// The station address drawn for this boot; `None` scans only.
    pub addr: Option<[u8; 6]>,
    /// The antennas both the firmware's PHY configuration and the NVM allow.
    pub tx_ant: u8,
    pub rx_ant: u8,
    /// The firmware runs every join command at the layout this driver
    /// encodes (`station::ids`).
    pub join_api: bool,
}
