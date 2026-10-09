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

//! Waking an application processor: INIT, then STARTUP twice.
//!
//! The sequence itself is `plan::startup_sequence`, checked on the host
//! against the SDM. This file only sends it: each command through the one ICR
//! writer, each wait in calibrated time, and the xAPIC delivery status polled
//! after every send, as Linux's `wakeup_secondary_cpu_via_init` does.

use super::constants::*;
use super::error::{ApicError, ApicResult};
use super::ipi_basic::{icr_write, wait_icr_idle};
use super::mmio::{mmio_r32, mmio_w32};
use super::plan::{
    dest_reachable, needs_init_deassert, park_icr_low, startup_sequence, StartupStep,
    INIT_DEASSERT_DELAY_US,
};
use super::state::*;
use core::sync::atomic::Ordering;

/// ESR bits that mean a send went wrong: send checksum, send accept, and
/// send illegal vector (bits 0, 2, 5) plus the receive side and illegal
/// register address. Bit 4 (receive accept, P6 only) is noise, as in Linux.
const ESR_SEND_ERRORS: u32 = 0xEF;

/// Send `apic_id` the INIT-SIPI-SIPI walk with the trampoline at page
/// `start_page`. Returns once the commands are sent and the SDM's waits have
/// passed; whether the AP actually runs is for the caller to wait on.
///
/// `apic_id` is the MADT's id for that processor, never a cpu number.
pub fn start_ap(apic_id: u32, start_page: u8) -> ApicResult<()> {
    let x2 = X2APIC_MODE.load(Ordering::Acquire);
    if !dest_reachable(apic_id, x2) {
        return Err(ApicError::DestinationUnreachable);
    }
    let version = super::ops_core::version();
    let (steps, count) = startup_sequence(start_page, needs_init_deassert(x2, version));

    clear_esr();
    for step in &steps[..count] {
        match *step {
            StartupStep::Send(low) => send_and_settle(apic_id, low, x2)?,
            StartupStep::DelayUs(us) => delay_us(us),
        }
    }
    if take_esr() & ESR_SEND_ERRORS != 0 {
        return Err(ApicError::SendRejected);
    }
    Ok(())
}

/// Put `apic_id` back into wait-for-SIPI. Used on an AP that was woken and
/// never answered, before the trampoline is rewritten for the next AP. INIT
/// stops it wherever it is in the trampoline, and it holds nothing there that
/// anyone else waits on. Returns once INIT has had the SDM's 10 ms to land.
pub fn park_ap(apic_id: u32) -> ApicResult<()> {
    let x2 = X2APIC_MODE.load(Ordering::Acquire);
    if !dest_reachable(apic_id, x2) {
        return Err(ApicError::DestinationUnreachable);
    }
    send_and_settle(apic_id, park_icr_low(), x2)?;
    delay_us(INIT_DEASSERT_DELAY_US);
    Ok(())
}

fn send_and_settle(apic_id: u32, low: u32, x2: bool) -> ApicResult<()> {
    if !icr_write(apic_id, low) {
        return Err(ApicError::IcrBusy);
    }
    if !x2 && !wait_icr_idle() {
        return Err(ApicError::IcrBusy);
    }
    Ok(())
}

/// Clear what the ESR has latched. The register must be written before it is
/// read, and only zero may be written in x2APIC mode.
fn clear_esr() {
    if X2APIC_MODE.load(Ordering::Acquire) {
        wrmsr(IA32_X2APIC_ESR, 0);
    } else {
        mmio_w32(LAPIC_ESR, 0);
    }
}

/// Latch the errors since `clear_esr` and read them.
fn take_esr() -> u32 {
    clear_esr();
    if X2APIC_MODE.load(Ordering::Acquire) {
        rdmsr(IA32_X2APIC_ESR) as u32
    } else {
        mmio_r32(LAPIC_ESR)
    }
}

fn delay_us(us: u32) {
    let freq = crate::sys::timer::tsc::tsc_frequency();
    if freq == 0 {
        port_delay_us(us);
        return;
    }
    let ticks = (freq as u128 * us as u128 / 1_000_000u128) as u64;
    let start = crate::sys::timer::tsc::rdtsc();
    while crate::sys::timer::tsc::rdtsc().wrapping_sub(start) < ticks {
        core::hint::spin_loop();
    }
}

/// The wait before the counter rate is known. A read of the POST port takes
/// about a microsecond on every chipset since the ISA bus, which is why Linux
/// uses it for `io_delay`; one read per microsecond is the right count. This
/// used two thousand, which turned the 10 ms INIT wait into twenty seconds.
fn port_delay_us(us: u32) {
    for _ in 0..us {
        // SAFETY: I/O port 0x80 is the legacy POST debug port; reading it
        // has no effect beyond the bus cycle.
        unsafe {
            core::arch::asm!(
                "in al, dx",
                in("dx") 0x80u16,
                out("al") _,
                options(nostack, nomem, preserves_flags)
            );
        }
    }
}
