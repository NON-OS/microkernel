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

use crate::xhci::{disable_slot, enable_slot, PortSnapshot};

use super::bind::bind;
use super::devices::Devices;
use super::types::Outcome;

/// Address the device on `snap`'s port and bind its HID interfaces or
/// bring it up as a hub. A device this driver does not keep has its slot
/// given back, so the class driver it belongs to can address it.
pub(super) fn configure_port(xhci_port: u32, snap: PortSnapshot, devs: &mut Devices) -> Outcome {
    let Ok(slot) = enable_slot(xhci_port) else {
        return Outcome::Failed;
    };
    let outcome = bind(xhci_port, slot, snap, devs);
    let kept = devs.holds(slot);
    if !kept {
        disable_slot(xhci_port, slot);
    }
    match outcome {
        Outcome::Bound if !kept => Outcome::Failed,
        other => other,
    }
}
