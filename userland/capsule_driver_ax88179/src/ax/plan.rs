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

//! The bring-up, in order, as data: each write named by what a refusal of
//! it stops, and each wait fixed. bring_up.rs runs it; the proofs hold it
//! to the order Linux v6.12's reset path gives the chip (read for
//! behaviour). Left out: the factory address (never read), checksum
//! offload (turned off), wake-on-LAN, the LED and auto-detach settings
//! from the EEPROM, and anything the receive path cannot bound.

use super::regs::*;
use super::step::{mac, phy, Step};

/// The bulk-in queue: aggregation on (07h), its timer (004Fh, the shortest
/// Linux sets), its size (2: a transfer of at most (2 + 2) KiB as Linux
/// sizes its buffer from it, so one bulk IN of BULK_MAX holds it), and the
/// gap (FFh). Linux's own sizes reach 26 KiB, past what one IN here takes.
pub const BULK_IN: [u8; 5] = [0x07, 0x4F, 0x00, 0x02, 0xFF];
/// Every multicast group: the stack keeps no list to hash.
const ALL_GROUPS: [u8; 8] = [0xFF; 8];
/// The medium until the PHY reports one: gigabit, full duplex, both pauses.
pub const MEDIUM_FIRST: u16 = M_RECEIVE | M_TX_PAUSE | M_RX_PAUSE | M_FULL | M_GIGA;

pub fn plan(station: [u8; 6]) -> [(&'static str, Step); 21] {
    [
        ("PHY reset refused", mac(POWER_RESET, &0u16.to_le_bytes())),
        ("PHY power refused", mac(POWER_RESET, &PHY_POWERED.to_le_bytes())),
        ("", Step::Pause(500)),
        ("clocks refused", mac(CLOCKS, &[CLOCKS_ON])),
        ("", Step::Pause(200)),
        ("station address refused", mac(STATION, &station)),
        ("bulk-in queue refused", mac(BULK_IN_QUEUE, &BULK_IN)),
        ("pause levels refused", mac(PAUSE_LOW, &[0x34])),
        ("pause levels refused", mac(PAUSE_HIGH, &[0x52])),
        ("checksum offload refused", mac(RX_CHECKSUM, &[0])),
        ("checksum offload refused", mac(TX_CHECKSUM, &[0])),
        ("group table refused", mac(GROUP_TABLE, &ALL_GROUPS)),
        ("receive control refused", mac(RX_CONTROL, &RX_ON.to_le_bytes())),
        ("medium refused", mac(MEDIUM, &MEDIUM_FIRST.to_le_bytes())),
        // Energy-efficient Ethernet off: the chip's page 3 setting, then
        // the advertisement (MMD 7, register 60) cleared through the window.
        ("EEE refused", phy(PHY_PAGE, 3)),
        ("EEE refused", phy(PHY_EEE, 0x3246)),
        ("EEE refused", phy(PHY_PAGE, 0)),
        ("EEE refused", phy(PHY_MMD_CTRL, 7)),
        ("EEE refused", phy(PHY_MMD_DATA, 60)),
        ("EEE refused", phy(PHY_MMD_CTRL, 0x4007)),
        ("EEE refused", phy(PHY_MMD_DATA, 0)),
    ]
}
