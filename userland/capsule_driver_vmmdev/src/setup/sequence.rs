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

//! The bring-up: the grants; the event page checked to be VMMDevMemory;
//! the host's version (a host that does not answer is no VMMDev); the
//! guest info; the events narrowed to the mouse's; no capabilities; any
//! event already pending cleared; and, last, mouse integration on. Turning
//! it on is last because the host then stops moving the PS/2 pointer: a
//! bring-up that failed after it would leave the guest without one.

use super::claim::claim;
use super::driver::Driver;
use crate::constants::mouse::{GUEST_CAN_ABSOLUTE, NEW_PROTOCOL};
use crate::constants::req::{MEMORY_SIZE_MIN, MEMORY_VERSION};
use crate::discover::Found;
use crate::regs::{Memory, Port};
use crate::req::{self, Page};

pub fn run(dev: Found) -> Result<Driver, &'static str> {
    let held = claim(dev.device_id)?;
    let back = |e: &'static str| {
        held.grants.release();
        e
    };
    let memory = Memory::new(held.memory.user_va);
    let size = memory.size();
    if memory.version() != MEMORY_VERSION
        || size < MEMORY_SIZE_MIN
        || size as u64 > dev.memory_bytes
    {
        return Err(back("event page is not VMMDevMemory"));
    }
    let mut page = Page::new(held.page.user_va, held.page.device_addr as u32, Port::new(held.port));
    let host = req::host_version(&mut page).map_err(|_| back("no answer to requests"))?;
    req::report_guest_info(&mut page).map_err(|_| back("guest info refused"))?;
    req::set_event_filter(&mut page).map_err(|_| back("event filter refused"))?;
    req::set_no_capabilities(&mut page).map_err(|_| back("capabilities refused"))?;
    req::acknowledge(&mut page).map_err(|_| back("events refused"))?;
    req::set_mouse(&mut page, GUEST_CAN_ABSOLUTE | NEW_PROTOCOL)
        .map_err(|_| back("mouse integration refused"))?;
    Ok(Driver { memory, page, host, last: None })
}
