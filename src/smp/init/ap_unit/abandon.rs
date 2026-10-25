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

//! Giving up on an AP that never claimed its start.

use crate::smp::{CpuDescriptor, CpuState};

/// Take the AP's claim and park it. False when the AP claimed first, in which
/// case it is running and must be waited for instead.
pub(super) fn abandon(ap: &CpuDescriptor, apic_id: u32) -> bool {
    if !ap.boot_claim.bsp_abandon() {
        return false;
    }
    // The AP can no longer pass its claim, so nothing else writes this state.
    ap.set_state(CpuState::Offline);
    let _ = reset(apic_id);
    true
}

/// INIT to `apic_id`: the core stops wherever it is and holds in
/// wait-for-SIPI with an empty TLB.
pub(super) fn reset(apic_id: u32) -> Result<(), &'static str> {
    crate::arch::x86_64::interrupt::apic::park_ap(apic_id).map_err(|e| e.as_str())
}
