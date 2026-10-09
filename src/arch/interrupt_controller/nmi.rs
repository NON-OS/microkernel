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

//! Raising an NMI on other cpus. Only the x86_64 local APIC has an NMI
//! command; on the other backends nothing is sent and the callers fall back
//! to the vector.

/// Raise an NMI on the cpu at `apic_id`. `false` means nothing was sent.
#[cfg(target_arch = "x86_64")]
pub(crate) fn nmi_one(apic_id: u32) -> bool {
    crate::arch::x86_64::interrupt::apic::nmi_one(apic_id)
}

/// Raise an NMI on every cpu but this one. `false` means nothing was sent.
#[cfg(target_arch = "x86_64")]
pub(crate) fn nmi_others() -> bool {
    crate::arch::x86_64::interrupt::apic::nmi_others()
}

#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn nmi_one(_apic_id: u32) -> bool {
    false
}

#[cfg(not(target_arch = "x86_64"))]
pub(crate) fn nmi_others() -> bool {
    false
}
