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

//! Whether a caller may hear about a broker device by id, whatever bus it
//! sits on: the same claim test for a PCI function and an ACPI-bus device.

use super::super::claim;
use super::claim_holder::Asker;

/// Whether the asker may hear about the device: a look always may, a
/// holder only for the device it holds. One claim test for every bus.
pub fn answers(asker: Asker, device_id: u64) -> bool {
    match asker {
        Asker::Anyone => true,
        Asker::Holder(pid) => claim::lookup(device_id).map(|c| c.pid) == Some(pid),
    }
}
