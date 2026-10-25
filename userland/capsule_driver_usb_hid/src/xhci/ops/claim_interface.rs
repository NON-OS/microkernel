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
//! An interface this driver drives, told to the host, so another class
//! driver may share the device: a composite key's FIDO interface beside the
//! keyboard bound here. The host keeps the slot until both give it back.

use crate::xhci::call::call;
use crate::xhci::wire::{HDR_LEN, OP_CLAIM_INTERFACE, STATUS_LEN};

pub fn claim_interface(xhci_port: u32, slot: u8, port: u8, interface: u8) {
    let mut resp = [0u8; HDR_LEN + STATUS_LEN];
    // A host without the op refuses it; the device is then not shared.
    let _ = call(xhci_port, OP_CLAIM_INTERFACE, &[slot, port, interface], &mut resp);
}
