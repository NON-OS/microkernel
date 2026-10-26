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

//! One AP started, and every way that can end said.

use super::super::boot_inputs::ApBootInputs;

/// Whether the AP at `apic_id` came up as `cpu_id`. A start that fails before
/// anything is sent, its stack or its boot context, says why here; the ones
/// that fail after INIT and STARTUP say why in `ap_unit`.
pub(super) fn start_one(cpu_id: usize, apic_id: u32, boot: &ApBootInputs) -> bool {
    match super::super::ap_unit::start(cpu_id, apic_id, boot) {
        Ok(up) => up,
        Err(why) => {
            let mut l = crate::sys::serial::Line::new();
            l.str(b"[SMP] ap=").dec(cpu_id as u64).str(b" apic=").dec(u64::from(apic_id));
            l.str(b" not started: ").str(why.as_bytes());
            l.end();
            false
        }
    }
}
