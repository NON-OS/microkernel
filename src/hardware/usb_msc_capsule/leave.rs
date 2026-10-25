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

//! This driver's leg of the leave phase (block_device::leave): a SYNCHRONIZE
//! CACHE through the flush op, which a slow stick may take seconds over.

use alloc::vec;

use super::protocol::{decode_response, encode_request, OP_BLK_FLUSH};
use super::transport::{wire_parts, SEQ};
use crate::hardware::block_device::leave::{Leg, Wire};
use crate::services::lifecycle::transport::next_request_id;

const BUDGET_MS: u64 = 8_000;

pub(crate) fn leave_leg() -> Option<Leg> {
    let (lock, sender, reply_inbox) = wire_parts();
    let state = crate::userspace::capsule_driver_usb_msc::shared_state();
    let wire = Wire { lock, sender, reply_inbox, state, decode: decode_response };
    let id = next_request_id(&SEQ);
    Leg::new("driver.usb_msc0", BUDGET_MS, wire, vec![(id, encode_request(OP_BLK_FLUSH, id, &[]))])
}
