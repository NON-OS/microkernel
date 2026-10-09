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

//! The arithmetic of the local APIC, apart from the registers it lands in.
//!
//! Everything here is a pure function of values read from the part or from
//! the MADT, so it is included by path into the host proofs and checked there
//! against the Intel SDM (vol. 3, ch. 10/11 and the MP init algorithm in
//! ch. 8/9). Nothing here touches hardware, takes a lock or names a crate
//! path, which is what lets it build on the host.

/// IA32_APIC_BASE bit 11, the global enable.
pub const BASE_ENABLE: u64 = 1 << 11;
/// IA32_APIC_BASE bit 10, x2APIC mode (EXTD).
pub const BASE_EXTD: u64 = 1 << 10;
/// IA32_APIC_BASE bits 12 and up hold the physical page of the xAPIC window.
/// The architectural limit is MAXPHYADDR, never above bit 51.
const BASE_ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

/// ICR low word fields (SDM 10.6.1, "Interrupt Command Register").
pub const ICR_DM_INIT: u32 = 0b101 << 8;
pub const ICR_DM_STARTUP: u32 = 0b110 << 8;
pub const ICR_DELIVERY_PENDING: u32 = 1 << 12;
pub const ICR_LEVEL_ASSERT: u32 = 1 << 14;
pub const ICR_TRIGGER_LEVEL: u32 = 1 << 15;

/// The largest physical destination an xAPIC ICR or an I/O APIC redirection
/// entry can name. The field is 8 bits and 0xFF is the broadcast id, so an
/// id above this is not a CPU an xAPIC write can reach. Truncating it does
/// not fail safe either: 0x100 becomes 0x00, which is usually the boot CPU,
/// and an INIT sent there resets the machine's own boot processor.
pub const XAPIC_MAX_DEST: u32 = 0xFE;

/// The 10 ms the SDM's MP initialisation example waits after INIT, and the
/// 200 us it waits after each STARTUP. Linux keeps the INIT wait for every
/// part older than family 6; it is harmless on newer ones, which ignore it.
pub const INIT_DEASSERT_DELAY_US: u32 = 10_000;
pub const STARTUP_DELAY_US: u32 = 200;

/// Local APIC version register values below this are the discrete 82489DX,
/// the only part that needs the INIT level de-assert.
const INTEGRATED_APIC_MIN_VERSION: u32 = 0x10;

/// LVT fields used for LINT0/LINT1.
pub const LVT_MASKED: u32 = 1 << 16;
pub const LVT_DM_NMI: u32 = 0b100 << 8;
pub const LVT_ACTIVE_LOW: u32 = 1 << 13;

/// Physical address of the xAPIC register page, as IA32_APIC_BASE names it.
/// Firmware may relocate it, so `0xFEE0_0000` is a default and not a fact.
#[inline]
pub const fn base_phys(apic_base_msr: u64) -> u64 {
    apic_base_msr & BASE_ADDR_MASK
}

/// Whether `apic_id` is reachable as a physical destination in this mode.
#[inline]
pub const fn dest_reachable(apic_id: u32, x2apic: bool) -> bool {
    x2apic || apic_id <= XAPIC_MAX_DEST
}

/// The x2APIC ICR is one 64-bit MSR: the 32-bit destination in the high half
/// and the command in the low half (SDM 10.12.9).
#[inline]
pub const fn x2apic_icr(dest: u32, low: u32) -> u64 {
    ((dest as u64) << 32) | low as u64
}

/// The xAPIC ICR high word for a physical destination, or `None` when the id
/// does not fit the 8-bit field.
#[inline]
pub const fn xapic_icr_high(dest: u32) -> Option<u32> {
    if dest <= XAPIC_MAX_DEST {
        Some(dest << 24)
    } else {
        None
    }
}

/// One step of the INIT-SIPI-SIPI walk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartupStep {
    /// Write this ICR low word to the target, then wait for the send to
    /// finish (xAPIC delivery status; x2APIC has none to wait for).
    Send(u32),
    /// Wait this many microseconds of calibrated time.
    DelayUs(u32),
}

/// Whether to follow INIT with a level de-assert. Only the discrete 82489DX
/// needs it, and x2APIC does not support the encoding at all, so a modern
/// part never receives one.
#[inline]
pub const fn needs_init_deassert(x2apic: bool, lapic_version: u32) -> bool {
    !x2apic && (lapic_version & 0xFF) < INTEGRATED_APIC_MIN_VERSION
}

/// The universal startup algorithm (SDM vol. 3, "MP Initialization Protocol
/// Algorithm"): INIT, 10 ms, optional de-assert, then STARTUP twice 200 us
/// apart with the trampoline page as the vector. `start_page` is the
/// trampoline's physical address shifted right by 12, so the trampoline must
/// be page aligned and below 1 MiB.
///
/// The INIT is the SDM's edge-triggered assert (`0x4500`). The de-assert is
/// Linux's level-triggered encoding (`0x8500`): level 0, trigger level.
pub fn startup_sequence(start_page: u8, deassert: bool) -> ([StartupStep; 6], usize) {
    let init = ICR_DM_INIT | ICR_LEVEL_ASSERT;
    let sipi = ICR_DM_STARTUP | ICR_LEVEL_ASSERT | start_page as u32;
    let mut steps = [StartupStep::DelayUs(0); 6];
    let mut n = 0;
    steps[n] = StartupStep::Send(init);
    n += 1;
    steps[n] = StartupStep::DelayUs(INIT_DEASSERT_DELAY_US);
    n += 1;
    if deassert {
        steps[n] = StartupStep::Send(ICR_DM_INIT | ICR_TRIGGER_LEVEL);
        n += 1;
    }
    steps[n] = StartupStep::Send(sipi);
    n += 1;
    steps[n] = StartupStep::DelayUs(STARTUP_DELAY_US);
    n += 1;
    steps[n] = StartupStep::Send(sipi);
    n += 1;
    (steps, n)
}

/// The ICR low word that parks a CPU back in wait-for-SIPI. Sent to an AP that
/// never answered, before the trampoline is rewritten for the next one, so a
/// late riser cannot read another CPU's stack out of it.
#[inline]
pub const fn park_icr_low() -> u32 {
    ICR_DM_INIT | ICR_LEVEL_ASSERT
}

/// The writes that take IA32_APIC_BASE from `current` to enabled in the
/// requested mode, in order.
///
/// The SDM (10.12.5, "x2APIC State Transitions") allows xAPIC to x2APIC
/// directly but not the reverse: leaving x2APIC must pass through disabled
/// (EN=0, EXTD=0), and EN=0 with EXTD=1 is an invalid state that faults.
/// INIT does not change the mode, so an AP that firmware left in x2APIC is
/// still there when it reaches the kernel, and writing the BSP's xAPIC choice
/// over it straight would #GP. Going from disabled to x2APIC is done through
/// xAPIC as well, which every revision of the SDM accepts.
pub fn base_transition(current: u64, want_x2apic: bool) -> ([u64; 3], usize) {
    let addr = current & !(BASE_ENABLE | BASE_EXTD);
    let enabled = current & BASE_ENABLE != 0;
    let extd = current & BASE_EXTD != 0;
    let mut out = [0u64; 3];
    let mut n = 0;
    match (enabled && extd, want_x2apic) {
        (true, true) => {}
        (true, false) => {
            out[n] = addr;
            n += 1;
            out[n] = addr | BASE_ENABLE;
            n += 1;
        }
        (false, want) => {
            if extd {
                // EN=0, EXTD=1 is invalid; clear it first.
                out[n] = addr;
                n += 1;
            }
            if !enabled || extd {
                out[n] = addr | BASE_ENABLE;
                n += 1;
            }
            if want {
                out[n] = addr | BASE_ENABLE | BASE_EXTD;
                n += 1;
            }
        }
    }
    (out, n)
}

/// One MADT Local APIC NMI (type 4) or Local x2APIC NMI (type 0xA) entry.
/// `processor_uid` is `u32::MAX` for "all processors".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LintNmi {
    pub processor_uid: u32,
    pub lint: u8,
    pub flags: u16,
}

/// The LINT0 and LINT1 LVT values for one CPU.
///
/// Both start masked: this kernel routes devices through the I/O APIC, so the
/// 8259's INTR on LINT0 must not reach a CPU, and on an AP it never should.
/// Each MADT NMI entry naming this processor, or all processors, turns its pin
/// into an NMI input with the polarity the entry gives (MPS INTI flags bits
/// 1:0, `11` active low). NMI delivery is edge by definition, so the trigger
/// bits are not used. With no entry for this CPU, the boot CPU gets the PC
/// default of NMI on LINT1 and an AP leaves it masked, as Linux does.
pub fn lint_lvts(nmis: &[LintNmi], my_uid: Option<u32>, is_bsp: bool) -> (u32, u32) {
    let mut lint = [LVT_MASKED, LVT_MASKED];
    let mut matched = false;
    for nmi in nmis {
        let mine = nmi.processor_uid == u32::MAX || Some(nmi.processor_uid) == my_uid;
        if !mine || nmi.lint > 1 {
            continue;
        }
        let mut v = LVT_DM_NMI;
        if nmi.flags & 0b11 == 0b11 {
            v |= LVT_ACTIVE_LOW;
        }
        lint[nmi.lint as usize] = v;
        matched = true;
    }
    if !matched && is_bsp {
        lint[1] = LVT_DM_NMI;
    }
    (lint[0], lint[1])
}

/// Which APIC id the running CPU has, from CPUID alone.
///
/// Leaf 1 EBX[31:24] is the initial xAPIC id and only 8 bits wide. On a part
/// whose ids pass 255, or one in x2APIC mode, it is the low byte of the real
/// id and names some other CPU. Leaf 0x0B (and 0x1F, which carries the same
/// EDX) reports the full 32-bit x2APIC id; a level-0 subleaf with EBX[15:0]
/// zero means the leaf is not implemented, so leaf 1 is the answer then.
#[inline]
pub const fn apic_id_from_cpuid(
    max_basic_leaf: u32,
    leaf_b_ebx: u32,
    leaf_b_edx: u32,
    leaf1_ebx: u32,
) -> u32 {
    if max_basic_leaf >= 0x0B && leaf_b_ebx & 0xFFFF != 0 {
        leaf_b_edx
    } else {
        leaf1_ebx >> 24
    }
}

/// Whether the firmware's processor list holds an id that only x2APIC mode can
/// address, which obliges the kernel to run in x2APIC mode.
pub fn requires_x2apic(enabled_ids: &[u32]) -> bool {
    enabled_ids.iter().any(|id| *id > XAPIC_MAX_DEST)
}

/// The physical destination for a device interrupt routed without interrupt
/// remapping: an I/O APIC redirection entry and an MSI address both carry an
/// 8-bit destination, in x2APIC mode as much as in xAPIC. A CPU with a larger
/// id cannot be named there, and truncating the id lands the interrupt on
/// whatever CPU owns the low byte, or on none. So `preferred` is kept when it
/// fits, and otherwise the first of `others` that fits takes the interrupt.
/// `None` means no CPU can be named at all.
pub fn pick_irq_dest(preferred: u32, others: impl IntoIterator<Item = u32>) -> Option<u32> {
    if preferred <= XAPIC_MAX_DEST {
        return Some(preferred);
    }
    others.into_iter().find(|id| *id <= XAPIC_MAX_DEST)
}
