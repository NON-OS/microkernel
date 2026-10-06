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

use super::constants::*;
use super::mmio::{mmio_r32, mmio_w32};
use super::plan::{x2apic_icr, xapic_icr_high};
use super::state::*;
use core::sync::atomic::Ordering;

/*
 * Every interrupt command written below carries it. The bit is level-assert,
 * and the processor delivers a command without it only in the one case it
 * still means something, an INIT level de-assert; a fixed-delivery command
 * with the bit clear is dropped. Of the mode constants this file ORs together
 * it is also the only one that is not zero, so leaving it out produced a word
 * that was purely destination and vector and looked complete while sending
 * nothing. `ipi_ap.rs` passes it by hand, which is why AP startup worked while
 * every runtime IPI, including the TLB shootdown, silently went nowhere.
 */
const ICR_SEND: u64 = ICR_DELIV_FIXED | ICR_LEVEL_ASSERT | ICR_TRIG_EDGE;

/// How long an xAPIC send may stay pending before it is called stuck, in
/// microseconds of calibrated time. The same 100 ms Linux allows in
/// `safe_apic_wait_icr_idle`. A healthy send clears in well under a
/// microsecond; this only bounds a broken one.
const ICR_IDLE_TIMEOUT_US: u64 = 100_000;

/// Polls allowed when the counter rate is not known yet. Each poll is an
/// uncached MMIO read, around a microsecond, so this is the same order.
const ICR_IDLE_FALLBACK_POLLS: u64 = 100_000;

/// Write one interrupt command. `dest` is ignored by the processor when `low`
/// carries a shorthand, and callers pass 0 then.
///
/// x2APIC: one 64-bit WRMSR with the full 32-bit destination, and no delivery
/// status to poll, because the x2APIC ICR has none (SDM 10.12.9). WRMSR to the
/// x2APIC range is not serialising and does not wait for earlier stores, so it
/// is fenced first, as Linux does with `weak_wrmsr_fence`: a reschedule or
/// shootdown that overtook the store announcing its work would find nothing
/// to do and the target would go back to sleep.
///
/// xAPIC: the destination has 8 bits, so an id it cannot hold is refused, not
/// truncated onto another CPU. The previous send must have left the ICR first,
/// and the low write is what sends, so the high word goes first.
///
/// False means nothing was sent.
pub(super) fn icr_write(dest: u32, low: u32) -> bool {
    if X2APIC_MODE.load(Ordering::Acquire) {
        // SAFETY: fences touch no memory beyond ordering it.
        unsafe { core::arch::asm!("mfence", "lfence", options(nostack, preserves_flags)) };
        wrmsr(IA32_X2APIC_ICR, x2apic_icr(dest, low));
        return true;
    }
    let shorthand = low as u64 & ICR_SH_OTHERS != 0;
    let high = if shorthand {
        0
    } else {
        match xapic_icr_high(dest) {
            Some(h) => h,
            None => return false,
        }
    };
    // The two halves go out with interrupts off: an interrupt handler that
    // sends its own IPI between them redirected this one to its target.
    crate::arch::run_without_interrupts(|| {
        if !wait_icr_idle() {
            return false;
        }
        mmio_w32(LAPIC_ICR_HIGH, high);
        mmio_w32(LAPIC_ICR_LOW, low);
        true
    })
}

pub fn ipi_self(vec: u8) -> bool {
    icr_write(0, (ICR_SEND | ICR_SH_SELF) as u32 | vec as u32)
}

pub fn ipi_one(apic_id: u32, vec: u8) -> bool {
    icr_write(apic_id, (ICR_SEND | ICR_DST_PHYSICAL | ICR_SH_NONE) as u32 | vec as u32)
}

pub fn ipi_all(vec: u8) -> bool {
    icr_write(0, (ICR_SEND | ICR_SH_ALL) as u32 | vec as u32)
}

pub fn ipi_others(vec: u8) -> bool {
    icr_write(0, (ICR_SEND | ICR_SH_OTHERS) as u32 | vec as u32)
}

/// Wait for the xAPIC delivery status bit to clear, bounded in calibrated
/// time rather than in loop turns, whose length depends on the part. True
/// when the ICR is free. Meaningless in x2APIC mode, where it is not called.
pub(super) fn wait_icr_idle() -> bool {
    if mmio_r32(LAPIC_ICR_LOW) & ICR_BUSY == 0 {
        return true;
    }
    let hz = crate::sys::timer::tsc::tsc_frequency();
    if hz == 0 {
        for _ in 0..ICR_IDLE_FALLBACK_POLLS {
            if mmio_r32(LAPIC_ICR_LOW) & ICR_BUSY == 0 {
                return true;
            }
            core::hint::spin_loop();
        }
        return false;
    }
    let budget = (hz as u128 * ICR_IDLE_TIMEOUT_US as u128 / 1_000_000) as u64;
    let start = crate::sys::timer::tsc::rdtsc();
    while mmio_r32(LAPIC_ICR_LOW) & ICR_BUSY != 0 {
        if crate::sys::timer::tsc::rdtsc().wrapping_sub(start) > budget {
            return false;
        }
        core::hint::spin_loop();
    }
    true
}
