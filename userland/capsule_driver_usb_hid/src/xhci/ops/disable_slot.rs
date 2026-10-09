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

use crate::xhci::call::call;
use crate::xhci::wire::{HDR_LEN, OP_DISABLE_SLOT, STATUS_LEN};

/// Give back a slot this driver will not keep, so the port is free for the
/// class driver its device belongs to. A refusal leaves the slot allocated
/// in the controller driver; there is nothing further to do from here.
pub fn disable_slot(xhci_port: u32, slot: u8) {
    let mut resp = [0u8; HDR_LEN + STATUS_LEN];
    let _ = call(xhci_port, OP_DISABLE_SLOT, &[slot], &mut resp);
}
