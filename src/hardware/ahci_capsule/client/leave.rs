// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! This driver's leg of the leave phase (block_device::leave): FLUSH CACHE
//! EXT, then STANDBY IMMEDIATE unless the machine restarts. The driver
//! bounds the two to 14 s (capsule_driver_ahci constants/power_off.rs,
//! POWER_OFF_MS) inside this budget, its KERNEL_POWER_OFF_MS.

use alloc::vec;

use super::super::protocol::{encode_request, OP_POWER_OFF, POWER_OFF_FLUSH_ONLY};
use super::seq::next_request_id;
use super::transport::wire;
use crate::hardware::block_device::leave::Leg;

const BUDGET_MS: u64 = 14_500;

pub(crate) fn leave_leg(restart: bool) -> Option<Leg> {
    let id = next_request_id();
    let flags = if restart { POWER_OFF_FLUSH_ONLY } else { 0 };
    Leg::new(
        "driver.ahci0",
        BUDGET_MS,
        wire(),
        vec![(id, encode_request(OP_POWER_OFF, flags, id, &[]))],
    )
}
