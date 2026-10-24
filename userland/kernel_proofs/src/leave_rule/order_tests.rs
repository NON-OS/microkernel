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

//! Where the leave phase runs: first in terminate(), before the other CPUs
//! stop and before the wipe, since the capsules that make the disks safe
//! must still be running; never on the panic path.

const TERMINATE: &str = include_str!("../../../../src/security/zerostate/terminate.rs");
const LEAVE: &str = include_str!("../../../../src/security/zerostate/leave.rs");

fn body_of_terminate() -> &'static str {
    let at = TERMINATE.find("pub(crate) fn terminate(").expect("terminate() is gone");
    &TERMINATE[at..]
}

#[test]
fn the_leave_phase_comes_before_the_stop_and_the_wipe() {
    let body = body_of_terminate();
    let leave = body.find("super::leave::run();").expect("terminate() no longer leaves");
    let stop = body.find("broadcast_ipi(Ipi::Stop)").expect("terminate() no longer stops the CPUs");
    let wipe = body.find("zerostate_shutdown_wipe()").expect("terminate() no longer wipes");
    assert!(leave < stop && stop < wipe, "leave {leave}, stop {stop}, wipe {wipe}");
}

#[test]
fn the_leave_phase_asks_the_disks() {
    assert!(LEAVE.contains("crate::hardware::block_device::leave::power_off_disks();"));
}

#[test]
fn the_panic_path_has_no_leave_phase() {
    let panic = include_str!("../../../../src/boot/panic/handler.rs");
    assert!(!panic.contains("leave::run") && !panic.contains("power_off_disks"));
}
