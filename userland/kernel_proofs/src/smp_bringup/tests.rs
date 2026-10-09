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

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use super::apic_constants as k;
use super::apic_plan::*;
use super::boot_claim::{budget_ticks, BootClaim};
use super::calibrate_consts::{LAPIC_TICKS_PER_MS_MAX, LAPIC_TICKS_PER_MS_MIN};
use super::idt_vectors as v;
use super::madt_plan::{plan, MadtCpu, PlanCounts};

/* ---- INIT-SIPI-SIPI (SDM vol. 3, MP initialisation protocol algorithm) ---- */

#[test]
fn startup_is_init_10ms_then_two_sipis_200us_apart() {
    let (steps, n) = startup_sequence(0x08, false);
    assert_eq!(
        &steps[..n],
        &[
            StartupStep::Send(0x4500),
            StartupStep::DelayUs(10_000),
            StartupStep::Send(0x4608),
            StartupStep::DelayUs(200),
            StartupStep::Send(0x4608),
        ]
    );
}

#[test]
fn deassert_only_for_the_discrete_apic_and_never_in_x2apic() {
    // 0x14/0x15 are integrated xAPICs (every part since P6), 0x0X the 82489DX.
    assert!(!needs_init_deassert(false, 0x0005_0014));
    assert!(!needs_init_deassert(false, 0x0006_0015));
    assert!(needs_init_deassert(false, 0x0003_0001));
    assert!(!needs_init_deassert(true, 0x0003_0001));
    let (steps, n) = startup_sequence(0x08, true);
    assert_eq!(steps[2], StartupStep::Send(0x8500), "level-triggered, level 0");
    assert_eq!(n, 6);
}

#[test]
fn sipi_vector_is_the_page_of_a_trampoline_below_1mib() {
    let tramp: u64 = 0x8000;
    assert_eq!(tramp & 0xFFF, 0, "page aligned");
    assert!(tramp < 0x10_0000, "below 1 MiB");
    let (steps, _) = startup_sequence((tramp >> 12) as u8, false);
    assert_eq!(steps[2], StartupStep::Send(ICR_DM_STARTUP | ICR_LEVEL_ASSERT | 0x08));
}

#[test]
fn park_is_an_init_assert() {
    assert_eq!(park_icr_low(), 0x4500);
}

/* ---- ICR destinations ---- */

#[test]
fn x2apic_icr_is_one_write_with_a_32_bit_destination() {
    assert_eq!(x2apic_icr(0x1_0042, 0x4041), 0x0001_0042_0000_4041);
    assert_eq!(x2apic_icr(u32::MAX - 1, 0x41) >> 32, 0xFFFF_FFFE);
}

#[test]
fn xapic_destination_is_never_truncated() {
    assert_eq!(xapic_icr_high(0), Some(0));
    assert_eq!(xapic_icr_high(0x4F), Some(0x4F00_0000));
    assert_eq!(xapic_icr_high(0xFE), Some(0xFE00_0000));
    // 0xFF is broadcast; 0x100 would truncate to the boot CPU.
    assert_eq!(xapic_icr_high(0xFF), None);
    assert_eq!(xapic_icr_high(0x100), None);
    assert!(dest_reachable(0x100, true));
    assert!(!dest_reachable(0x100, false));
}

/* ---- IA32_APIC_BASE (SDM 10.12.5, x2APIC state transitions) ---- */

const ADDR: u64 = 0xFEE0_0000;
const EN: u64 = BASE_ENABLE;
const X2: u64 = BASE_EXTD;

/// The states the SDM allows, and the moves between them: disabled ->
/// xAPIC, xAPIC -> x2APIC, x2APIC -> disabled, and staying put.
fn legal(from: u64, to: u64) -> bool {
    let s = |b: u64| (b & EN != 0, b & X2 != 0);
    let (a, b) = (s(from), s(to));
    if b == (false, true) {
        return false;
    }
    a == b
        || matches!(
            (a, b),
            ((false, false), (true, false))
                | ((true, false), (true, true))
                | ((true, true), (false, false))
                | ((false, true), (false, false))
        )
}

fn walk(current: u64, want_x2: bool) -> u64 {
    let (w, n) = base_transition(current, want_x2);
    let mut at = current;
    for &next in &w[..n] {
        assert!(legal(at, next), "illegal {at:#x} -> {next:#x}");
        assert_eq!(base_phys(next), ADDR, "address kept");
        at = next;
    }
    at
}

#[test]
fn every_start_state_reaches_the_wanted_mode_legally() {
    for start in [ADDR, ADDR | EN, ADDR | EN | X2, ADDR | X2] {
        for want in [false, true] {
            let end = walk(start | 0x100, want) & !0x100;
            let expect = if want { ADDR | EN | X2 } else { ADDR | EN };
            assert_eq!(end & (EN | X2), expect & (EN | X2), "from {start:#x} want_x2={want}");
        }
    }
}

#[test]
fn firmware_x2apic_on_an_ap_is_left_alone_when_wanted() {
    let (_, n) = base_transition(ADDR | EN | X2, true);
    assert_eq!(n, 0);
}

#[test]
fn base_comes_from_the_msr_not_a_constant() {
    assert_eq!(base_phys(0xFED0_0000 | EN | 0x100), 0xFED0_0000);
    assert_eq!(base_phys(0x0000_00FF_FEE0_0900), 0x0000_00FF_FEE0_0000);
}

/* ---- LINT0/LINT1 from MADT NMI entries (types 4 and 0xA) ---- */

#[test]
fn lint_defaults_mask_lint0_and_give_the_bsp_nmi_on_lint1() {
    assert_eq!(lint_lvts(&[], Some(0), true), (LVT_MASKED, LVT_DM_NMI));
    assert_eq!(lint_lvts(&[], Some(1), false), (LVT_MASKED, LVT_MASKED));
}

#[test]
fn lint_follows_madt_entries_for_this_uid_or_all() {
    let all = LintNmi { processor_uid: u32::MAX, lint: 1, flags: 0b0101 };
    assert_eq!(lint_lvts(&[all], Some(3), false), (LVT_MASKED, LVT_DM_NMI));

    // Per-processor entries, as HP and Lenovo firmware lists them.
    let per = [
        LintNmi { processor_uid: 1, lint: 1, flags: 0b1111 },
        LintNmi { processor_uid: 2, lint: 1, flags: 0b0101 },
    ];
    assert_eq!(lint_lvts(&per, Some(1), true), (LVT_MASKED, LVT_DM_NMI | LVT_ACTIVE_LOW));
    assert_eq!(lint_lvts(&per, Some(2), false), (LVT_MASKED, LVT_DM_NMI));
    // A CPU no entry names keeps the defaults.
    assert_eq!(lint_lvts(&per, Some(9), false), (LVT_MASKED, LVT_MASKED));
    // A nonsense pin is ignored.
    let bad = LintNmi { processor_uid: u32::MAX, lint: 7, flags: 0 };
    assert_eq!(lint_lvts(&[bad], None, false), (LVT_MASKED, LVT_MASKED));
}

/* ---- APIC id from CPUID ---- */

#[test]
fn apic_id_uses_the_32_bit_x2apic_id_when_leaf_0b_exists() {
    // x2APIC id 0x104 on a many-core part: leaf 1 only knows 0x04.
    assert_eq!(apic_id_from_cpuid(0x1F, 0x0002, 0x104, 0x0410_0800), 0x104);
    // Hybrid E-core id 0x4F, same either way.
    assert_eq!(apic_id_from_cpuid(0x20, 0x0001, 0x4F, 0x4F10_0800), 0x4F);
    // No leaf 0x0B (or an empty one): fall back to leaf 1.
    assert_eq!(apic_id_from_cpuid(0x0A, 0, 0, 0x0310_0800), 3);
    assert_eq!(apic_id_from_cpuid(0x0D, 0, 0xDEAD, 0x0210_0800), 2);
}

#[test]
fn x2apic_is_required_only_past_0xfe() {
    assert!(!requires_x2apic(&[0, 2, 4, 6, 0x4F, 0xFE]));
    assert!(requires_x2apic(&[0, 0xFF]));
    assert!(requires_x2apic(&[0, 0x100]));
}

/* ---- device interrupt destinations without remapping ---- */

#[test]
fn device_irqs_go_to_a_cpu_an_8_bit_field_can_name() {
    assert_eq!(pick_irq_dest(0x10, [0]), Some(0x10));
    assert_eq!(pick_irq_dest(0x104, [0x100, 0x20, 0]), Some(0x20));
    assert_eq!(pick_irq_dest(0x104, [0x100, 0x101]), None);
    assert_eq!(pick_irq_dest(0xFF, [0]), Some(0), "0xFF is broadcast, not a CPU");
}

/* ---- vectors ---- */

#[test]
fn lapic_vectors_have_gates_and_do_not_alias_device_lines() {
    assert_eq!(k::VEC_SPURIOUS, v::VECTOR_APIC_SPURIOUS);
    assert_eq!(k::VEC_ERROR, v::VECTOR_APIC_ERROR);
    assert_eq!(k::VEC_THERMAL, v::VECTOR_APIC_THERMAL);
    for vec in [k::VEC_ERROR, k::VEC_THERMAL, k::VEC_SPURIOUS] {
        assert!(!(v::IRQ_VECTOR_START..=v::IRQ_VECTOR_END).contains(&vec));
    }
    assert_eq!(k::VEC_SPURIOUS & 0x0F, 0x0F, "SVR vector low nibble set on P6");
}

/* ---- the AP's claim against the boot CPU's give-up ---- */

#[test]
fn claim_has_one_winner() {
    let c = BootClaim::new();
    c.arm();
    assert!(c.ap_enter());
    assert!(!c.bsp_abandon(), "an entered AP is waited for, never parked");
    assert!(c.entered());

    let c = BootClaim::new();
    c.arm();
    assert!(c.bsp_abandon());
    assert!(!c.ap_enter(), "a late AP must park");
    assert!(!c.entered());
}

#[test]
fn claim_race_never_has_two_winners() {
    for _ in 0..2000 {
        let c = Arc::new(BootClaim::new());
        c.arm();
        let wins = Arc::new(AtomicU32::new(0));
        let (c2, w2) = (c.clone(), wins.clone());
        let ap = std::thread::spawn(move || {
            if c2.ap_enter() {
                w2.fetch_add(1, Ordering::SeqCst);
            }
        });
        if c.bsp_abandon() {
            wins.fetch_add(1, Ordering::SeqCst);
        }
        ap.join().unwrap();
        assert_eq!(wins.load(Ordering::SeqCst), 1);
    }
}

#[test]
fn timeouts_are_calibrated_time_with_a_floor() {
    assert_eq!(budget_ticks(1000, 1_100_000_000, 7), 1_100_000_000);
    assert_eq!(budget_ticks(1000, 0, 7), 7);
    assert_eq!(budget_ticks(1000, 999, 7), 7);
}

/* ---- which MADT entries become APs ---- */

fn lapic(id: u32) -> MadtCpu {
    MadtCpu { apic_id: id, enabled: true, x2apic_entry: false }
}
fn x2(id: u32) -> MadtCpu {
    MadtCpu { apic_id: id, enabled: true, x2apic_entry: true }
}
fn run(entries: &[MadtCpu], own: u32, x2mode: bool, max: usize) -> (Vec<u32>, PlanCounts) {
    let mut out = Vec::new();
    let c = plan(entries, own, x2mode, max, |id| out.push(id));
    (out, c)
}

#[test]
fn gemini_lake_four_cores_dense_ids() {
    let (aps, c) = run(&[lapic(0), lapic(2), lapic(4), lapic(6)], 0, false, 256);
    assert_eq!(aps, [2, 4, 6]);
    assert_eq!(c.accepted, 3);
}

#[test]
fn hybrid_alder_lake_sparse_ids_and_disabled_slots() {
    // 6P (HT) + 8E: P ids 0..11 in pairs, E ids 0x40..0x47, plus hotplug
    // slots firmware lists disabled.
    let mut e: Vec<MadtCpu> = (0..12).map(lapic).collect();
    e.extend((0x40..0x48).map(lapic));
    e.extend((0x60..0x64).map(|id| MadtCpu { apic_id: id, enabled: false, x2apic_entry: false }));
    let (aps, c) = run(&e, 0, false, 256);
    assert_eq!(aps.len(), 19);
    assert!(aps.contains(&0x47));
    assert_eq!(c.disabled, 4);
}

#[test]
fn bsp_not_first_and_not_zero() {
    let (aps, _) = run(&[lapic(0), lapic(1), lapic(8), lapic(9)], 8, false, 256);
    assert_eq!(aps, [0, 1, 9]);
}

#[test]
fn duplicate_lapic_and_x2apic_entries_start_once() {
    let e = [lapic(0), lapic(2), x2(0), x2(2), x2(4)];
    let (aps, c) = run(&e, 0, true, 256);
    assert_eq!(aps, [2, 4]);
    assert_eq!(c.duplicate, 1);
}

#[test]
fn placeholders_are_not_processors() {
    let (aps, c) = run(&[lapic(0), lapic(0xFF), x2(u32::MAX), lapic(1)], 0, true, 256);
    assert_eq!(aps, [1]);
    assert_eq!(c.invalid, 2);
}

#[test]
fn ids_past_0xfe_need_x2apic() {
    let e = [x2(0), x2(0x100), x2(0x101), lapic(3)];
    let (aps, c) = run(&e, 0, false, 256);
    assert_eq!(aps, [3]);
    assert_eq!(c.unaddressable, 2);
    let (aps, _) = run(&e, 0, true, 256);
    assert_eq!(aps, [0x100, 0x101, 3]);
}

#[test]
fn more_cpus_than_max_are_counted_not_overrun() {
    let e: Vec<MadtCpu> = (0..40).map(lapic).collect();
    let (aps, c) = run(&e, 0, false, 16);
    assert_eq!(aps.len(), 15, "15 APs plus the boot CPU is 16");
    assert_eq!(c.over_limit, 24);
}

/* ---- LAPIC timer calibration band ---- */

#[test]
fn calibration_band_admits_crystal_clocked_parts() {
    // Ticks per ms at divide-by-16: Gemini Lake 19.2 MHz crystal, Skylake
    // client 24 MHz, Alder Lake 38.4 MHz, 100 MHz bus, AMD Zen 100 MHz.
    for hz in [19_200_000u64, 24_000_000, 38_400_000, 100_000_000, 400_000_000] {
        let per_ms = hz / 16 / 1000;
        assert!(
            (LAPIC_TICKS_PER_MS_MIN..=LAPIC_TICKS_PER_MS_MAX).contains(&per_ms),
            "{hz} Hz -> {per_ms}/ms clamped"
        );
    }
}
