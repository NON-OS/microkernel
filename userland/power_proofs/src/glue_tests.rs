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

//! capsule_power's side, read from the source: both names land in one inbox
//! and a POFF frame goes to `power.off`'s handler; the sender is the shell's
//! process or the request is refused; only the rule's `Off` reaches the
//! kernel's AdminShutdown; the `power` port gates reboot and shutdown too.

const RUNNER: &str = include_str!("../../capsule_power/src/server/runner.rs");
const OFF: &str = include_str!("../../capsule_power/src/off/mod.rs");
const ROUTER: &str = include_str!("../../capsule_power/src/server/handlers/router.rs");
const CAPSULE_MK: &str = include_str!("../../capsule_power/Capsule.mk");

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing: {needle}"))
}

#[test]
fn a_poff_frame_goes_to_power_off_with_the_senders_standing() {
    assert!(RUNNER.contains("const SHELL: &[u8] = b\"desktop_shell\";"));
    assert!(RUNNER.contains("let from_shell = super::caller::runs(sender_pid, SHELL);"));
    let poff = at(RUNNER, "if crate::off::wire::is_poff(req) {");
    assert!(poff < at(RUNNER, "crate::off::requested(req, from_shell, &mut out_buf)"));
    assert!(poff < at(RUNNER, "route(&mut state, req, from_shell, &mut out_buf)"));
}

#[test]
fn only_the_rules_off_reaches_admin_shutdown() {
    assert_eq!(OFF.matches("mk_admin_shutdown()").count(), 1);
    let off = at(OFF, "Verdict::Off => {");
    assert!(at(OFF, "Verdict::Refuse(e) => e,") < off);
    assert!(off < at(OFF, "mk_admin_shutdown() as i32"));
    assert!(OFF.contains("match verdict(wire::well_formed(req), from_shell) {"));
}

#[test]
fn the_power_port_checks_the_sender_before_any_op() {
    let check = at(ROUTER, "if !op_allowed(req.op, from_shell) {");
    assert!(check < at(ROUTER, "OP_REBOOT => reboot::run"));
    assert!(check < at(ROUTER, "OP_SHUTDOWN => shutdown::run"));
}

#[test]
fn the_capsule_declares_power_off_on_its_port() {
    assert!(CAPSULE_MK.contains("CAPSULE_INSTANCE_ENDPOINTS := service:4446:power.off\n"));
}
