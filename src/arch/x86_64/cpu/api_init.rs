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

use super::error::CpuError;
use super::state;

#[inline]
pub fn init() -> Result<(), CpuError> {
    state::init()
}

#[inline]
pub unsafe fn init_ap(cpu_id: u16, apic_id: u32) -> Result<(), CpuError> {
    let r = unsafe { state::init_ap(cpu_id, apic_id) };
    // Whatever became of the per-CPU record, this CPU must see the
    // framebuffer through the same table as the boot CPU before it runs a
    // thread that presents.
    // SAFETY: the AP's bring-up, interrupts off, before its first thread.
    unsafe { crate::arch::x86_64::pat::mirror_on_ap() };
    r
}

#[inline]
pub fn is_initialized() -> bool {
    state::is_initialized()
}
