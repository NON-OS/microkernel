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

//! The channel message numbers, pinned to the protocol's own table: QEMU's
//! include/hw/hyperv/vmbus-proto.h (VMBUS_MSG_*), the same values as
//! Linux's `enum vmbus_channel_message_type`. The model once took them
//! from the core, so a wrong number passed both; it cannot now.

use crate::constants::*;
use crate::msg::relid_released;

#[test]
fn the_core_s_message_numbers_are_the_protocol_s() {
    assert_eq!(OFFERCHANNEL, 1);
    assert_eq!(RESCIND_CHANNELOFFER, 2);
    assert_eq!(REQUESTOFFERS, 3);
    assert_eq!(ALLOFFERS_DELIVERED, 4);
    assert_eq!(RELID_RELEASED, 13);
    assert_eq!(INITIATE_CONTACT, 14);
    assert_eq!(VERSION_RESPONSE, 15);
    assert_eq!(UNLOAD, 16);
    assert_eq!(UNLOAD_RESPONSE, 17);
}

#[test]
fn a_released_relid_is_message_thirteen() {
    let m = relid_released(7);
    assert_eq!(&m[..4], &13u32.to_le_bytes());
    assert_eq!(&m[8..12], &7u32.to_le_bytes());
}
