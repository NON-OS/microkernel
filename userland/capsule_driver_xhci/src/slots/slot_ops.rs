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

//! The requests the gate holds to a slot (gate.rs): every one whose first
//! byte is a slot id. Port, controller and Enable Slot requests name none;
//! Slot of Port names a port.

use crate::protocol::*;

/// The requests whose first byte is a slot id.
pub fn names_slot(op: u16) -> bool {
    matches!(
        op,
        OP_DISABLE_SLOT
            | OP_ADDRESS_DEVICE
            | OP_GET_DEVICE_DESCRIPTOR
            | OP_GET_CONFIG_DESCRIPTOR
            | OP_ALLOC_TRANSFER_RING
            | OP_CONTROL_TRANSFER
            | OP_INTERRUPT_IN
            | OP_INTERRUPT_OUT
            | OP_CONFIGURE_BULK
            | OP_CONFIGURE_BULK2
            | OP_CONFIGURE_STREAMS
            | OP_BULK_OUT
            | OP_BULK_IN
            | OP_BULK_IN_POLL
            | OP_RESET_BULK
            | OP_ADDRESS_ROUTED
            | OP_CONFIGURE_HUB
            | OP_CONFIGURE_ISOCH
            | OP_ISOCH_OUT
            | OP_ISOCH_IN
            | OP_CLAIM_INTERFACE
    )
}
