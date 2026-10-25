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

use super::constants::{ARCH_CAP_IBRS_ALL, SPEC_CTRL_IBRS, SPEC_CTRL_SSBD, SPEC_CTRL_STIBP};
use super::policy::{barrier_on_switch, rewrite_on_entry, spec_ctrl_bits};

const APPLY: &str =
    include_str!("../../../../src/security/hardening/spectre_mitigations/cpu_apply.rs");
const AP_SETUP: &str = include_str!("../../../../src/smp/ap/user_setup.rs");
const SWITCH: &str = include_str!("../../../../src/arch/x86_64/context/switch/dispatch.rs");

#[test]
fn the_bits_are_the_architectural_ones() {
    assert_eq!((SPEC_CTRL_IBRS, SPEC_CTRL_STIBP, SPEC_CTRL_SSBD), (1, 2, 4));
    assert_eq!(ARCH_CAP_IBRS_ALL, 1 << 1);
}

#[test]
fn every_offered_control_is_kept_and_nothing_else() {
    for i in 0..8u8 {
        let (ibrs, stibp, ssbd) = (i & 1 != 0, i & 2 != 0, i & 4 != 0);
        assert_eq!(spec_ctrl_bits(ibrs, stibp, ssbd), u64::from(i));
    }
}

#[test]
fn only_legacy_ibrs_is_written_again_on_entry() {
    assert!(rewrite_on_entry(true, false));
    assert!(!rewrite_on_entry(true, true));
    assert!(!rewrite_on_entry(false, false) && !rewrite_on_entry(false, true));
}

#[test]
fn a_barrier_between_processes_and_none_between_threads() {
    assert!(barrier_on_switch(true, 7, 9));
    assert!(!barrier_on_switch(true, 7, 7));
    // A CPU that has run no process yet takes it.
    assert!(barrier_on_switch(true, 0, 7));
    assert!(!barrier_on_switch(false, 7, 9));
}

#[test]
fn every_ap_applies_the_bits_and_every_switch_asks() {
    // Issue #425: the boot CPU's write never reached the APs.
    assert!(AP_SETUP.contains("spectre_mitigations::apply_on_this_cpu();"));
    assert!(SWITCH.contains("spectre_mitigations::switch_barrier(pcb.thread_group_id());"));
    // Firmware bits survive: the apply ors into what is there.
    assert!(APPLY.contains("rdmsr(MSR_IA32_SPEC_CTRL) | bits"));
}

#[test]
fn after_an_aborted_switch_the_next_switch_to_any_process_takes_the_barrier() {
    // An aborted switch leaves 0 on the CPU, not the group it was going to.
    for next in [1, 7, 9, u32::MAX] {
        assert!(barrier_on_switch(true, 0, next), "{next}");
    }
}
