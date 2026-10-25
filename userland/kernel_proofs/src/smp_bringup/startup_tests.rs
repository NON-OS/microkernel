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

use super::apic_plan::*;

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
