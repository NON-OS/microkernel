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

//! The batteries and AC adapters firmware describes, and what was last read
//! from each.

use alloc::vec::Vec;
use nonos_aml::namespace::Id;
use nonos_aml::power::{BatteryInfo, BatteryStatus};

pub struct Battery {
    pub id: Id,
    /// _STA's battery-present bit.
    pub present: bool,
    pub info: Option<BatteryInfo>,
    pub status: Option<BatteryStatus>,
    /// The _BST faults already said for this battery (`bst_told`).
    pub told: u8,
}

#[derive(Default)]
pub struct Power {
    pub batteries: Vec<Battery>,
    /// Each adapter and whether it supplies power, when _PSR answered.
    pub adapters: Vec<(Id, Option<bool>)>,
    /// The word last reported, and when (ms since boot).
    pub reported: Option<u64>,
    pub read_ms: u64,
    /// The kernel was told of a low battery, and it still is low.
    pub low_told: bool,
    /// The kernel was last told the battery is critical.
    pub critical: bool,
}
