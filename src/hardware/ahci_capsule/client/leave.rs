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

//! This driver's leg of the leave phase (block_device::leave): FLUSH CACHE EXT, then STANDBY IMMEDIATE, a disk spinning down among them.

use alloc::vec;

use super::super::protocol::{encode_request, OP_POWER_OFF};
use super::seq::next_request_id;
use super::transport::wire;
use crate::hardware::block_device::leave::Leg;

const BUDGET_MS: u64 = 12_000;

pub(crate) fn leave_leg() -> Option<Leg> {
    let id = next_request_id();
    Leg::new(
        "driver.ahci0",
        BUDGET_MS,
        wire(),
        vec![(id, encode_request(OP_POWER_OFF, 0, id, &[]))],
    )
}
