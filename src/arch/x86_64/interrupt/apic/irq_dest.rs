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

//! Choosing a CPU that a device interrupt can actually be sent to.
//!
//! Without interrupt remapping the I/O APIC and MSI destination is 8 bits, so
//! on a machine whose APIC ids pass 0xFE (many-core parts in x2APIC mode) a
//! route aimed at such a CPU is redirected to one that can be named, the boot
//! CPU first, rather than truncated. `plan::pick_irq_dest` is the rule.

use core::sync::atomic::Ordering;

use super::plan::pick_irq_dest;
use super::state::CACHED_ID;

pub fn device_irq_dest(preferred: u32) -> Option<u32> {
    let bsp = CACHED_ID.load(Ordering::Acquire);
    let online = (0..crate::smp::MAX_CPUS)
        .filter(|cpu| crate::smp::cpu_is_online(*cpu))
        .filter_map(crate::smp::get_cpu)
        .map(|d| d.get_apic_id());
    pick_irq_dest(preferred, core::iter::once(bsp).chain(online))
}
