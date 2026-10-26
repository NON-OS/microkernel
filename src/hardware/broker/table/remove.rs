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

//! A record taken out of the table when its device has left (the ACPI
//! broker's `depart`). Its id is not reused: ids only ever grow, so a
//! capsule holding the old one can never reach whatever arrives next.

use super::state::{retire, TABLE};
use crate::hardware::broker::DeviceRecord;

pub(in crate::hardware::broker) fn remove(device_id: u64) -> Option<DeviceRecord> {
    let mut table = TABLE.write();
    let at = table.iter().position(|r| r.device_id == device_id)?;
    retire(device_id);
    Some(table.remove(at))
}
