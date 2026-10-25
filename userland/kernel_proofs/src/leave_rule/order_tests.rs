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

//! Where the leave phase runs: in terminate() and terminate_critical(),
//! after the once-guard and before the other CPUs stop and before the wipe,
//! since the capsules that make the disks safe must still be running; never
//! on the panic path.

const TERMINATE: &str = include_str!("../../../../src/security/zerostate/terminate.rs");
const LEAVE: &str = include_str!("../../../../src/security/zerostate/leave.rs");

/// The text of `fn name(` up to the next item's start.
fn body_of(name: &str) -> &'static str {
    let at = TERMINATE.find(&format!("fn {name}(")).unwrap_or_else(|| panic!("{name}() is gone"));
    let rest = &TERMINATE[at..];
    let end =
        rest[1..].find("\nfn ").or_else(|| rest[1..].find("\npub")).map_or(rest.len(), |e| e + 1);
    &rest[..end]
}

#[test]
fn both_ways_out_go_through_the_one_end() {
    assert!(body_of("terminate").contains("end(off, TOTAL_MS)"));
    assert!(body_of("terminate_critical").contains("end(off, CRITICAL_TOTAL_MS)"));
}

#[test]
fn the_once_guard_then_the_leave_phase_come_before_the_stop_and_the_wipe() {
    let body = body_of("end");
    let once = body.find("!first(&ENDING)").expect("the once-guard is gone");
    let leave = body.find("super::leave::run(off, leave_ms);").expect("end() no longer leaves");
    let stop = body.find("broadcast_ipi(Ipi::Stop)").expect("end() no longer stops the CPUs");
    let wipe = body.find("zerostate_shutdown_wipe()").expect("end() no longer wipes");
    assert!(once < leave && leave < stop && stop < wipe, "{once} {leave} {stop} {wipe}");
}

#[test]
fn a_later_caller_parks_without_halting_its_cpu() {
    let park = body_of("park");
    assert!(park.contains("lifecycle::rest(round)"), "a later caller does not give its CPU back");
    assert!(!park.contains("halt") && !park.contains("disable_interrupts"), "{park}");
}

#[test]
fn the_leave_phase_asks_the_disks_and_a_restart_spins_none_down() {
    assert!(LEAVE.contains("power_off_disks(restart, total_ms)"));
    assert!(LEAVE.contains("matches!(off, PowerOff::Reboot)"));
}

#[test]
fn the_panic_path_has_no_leave_phase() {
    let panic = include_str!("../../../../src/boot/panic/handler.rs");
    assert!(!panic.contains("leave::run") && !panic.contains("power_off_disks"));
}
