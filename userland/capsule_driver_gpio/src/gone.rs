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

//! A message from sender 0 is read only as DGON, the kernel's notice that
//! a device left (`nonos_libc::dgon_parse`: exact tag, version and length);
//! anything else from sender 0 is dropped. When the device is one of the
//! controllers, its bank is let go at once: its grants are already
//! revoked, so nothing of it may be touched again. With no bank left the
//! capsule exits, and is seen exiting like any other.

use crate::bank::Bank;
use crate::say::line;
use alloc::format;
use alloc::vec::Vec;
use nonos_libc::{dgon_parse, mk_exit, EXIT_ABSENT};

pub fn kernel_message(banks: &mut Vec<Bank>, msg: &[u8]) {
    let Some(g) = dgon_parse(msg, 0) else { return };
    let before = banks.len();
    banks.retain(|b| b.record.device_id != g.device_id);
    if banks.len() == before {
        return;
    }
    line(&format!("controller device {:#x} left; its pins are gone", g.device_id));
    if banks.is_empty() {
        line("no controller left");
        mk_exit(EXIT_ABSENT);
    }
}
