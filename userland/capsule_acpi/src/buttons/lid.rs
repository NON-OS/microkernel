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

//! Reading the lids' _LID and telling the kernel, which tells the shell's
//! lock (F-91), when that changes: on each Notify, and again every
//! LID_POLL_MS, so a Notify firmware never sent is caught within that. The
//! state is only taken as told once the kernel has it, so a refused report
//! (the shell's input busy) is made again at the next reading. Any lid
//! open reads as open.

use super::state::Buttons;
use crate::boot::evaluate;
use crate::host::CapsuleHost;
use crate::sys::acpi_event::{acpi_event, LID};
use crate::sys::clock::now_ms;
use crate::sys::log::Line;
use alloc::vec::Vec;
use nonos_aml::button::lid_open;
use nonos_aml::load::Aml;

/// Often enough for a lock that follows the lid; _LID is one EC read.
const LID_POLL_MS: u64 = 2_000;

pub fn tick(b: &mut Buttons, aml: &mut Aml, host: &mut CapsuleHost) {
    let now = now_ms();
    if b.lids.is_empty() || now < b.next_lid_ms {
        return;
    }
    b.next_lid_ms = now.saturating_add(LID_POLL_MS);
    read_lid(b, aml, host);
}

pub fn read_lid(b: &mut Buttons, aml: &mut Aml, host: &mut CapsuleHost) {
    let mut open = None;
    for &lid in &b.lids {
        let Some(id) = aml.ns.child(lid, *b"_LID") else { continue };
        if let Some(o) = evaluate(aml, host, id, Vec::new()).ok().as_ref().and_then(lid_open) {
            open = Some(open.unwrap_or(false) || o);
        }
    }
    let Some(o) = open else { return };
    if b.lid_open != open && acpi_event(LID, u64::from(o)).is_ok() {
        b.lid_open = open;
        Line::new(b"lid ").str(if o { b"open" } else { b"closed" }).end();
    }
}
