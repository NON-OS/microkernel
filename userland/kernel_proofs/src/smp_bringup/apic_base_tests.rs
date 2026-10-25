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
