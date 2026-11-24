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

//! A launch has one role and proves its program once, after its family
//! exists: nothing decides a role before the guest, nothing asks to start
//! again, and the start proves and binds the program between the family's
//! spawn and the guest's first instruction, so the kernel records both the
//! program and its policy in the family that runs it.

const START: &str = include_str!("../../../../capsule_linux/src/linux/start.rs");
const START_GUEST: &str = include_str!("../../../../capsule_linux/src/linux/start_guest.rs");
const PROVE: &str = include_str!("../../../../capsule_linux/src/linux/start_guest/prove.rs");
const MOD: &str = include_str!("../../../../capsule_linux/src/linux/mod.rs");

/// The body of `fn name(` in `src`, to its closing brace at the margin.
fn body<'a>(src: &'a str, name: &str) -> &'a str {
    let from = &src[src.find(&format!("fn {name}(")).unwrap_or_else(|| panic!("{name} is gone"))..];
    &from[..from.find("\n}\n").expect("the function ends")]
}

#[test]
fn no_launch_decides_a_role_or_asks_to_start_again() {
    for gone in ["run_net", "net_role", "mk_linux_run_net", "LINUX_ASK_NET", "bind_role"] {
        assert!(!START.contains(gone) && !START_GUEST.contains(gone), "{gone} is back");
    }
    assert!(!MOD.contains("mod run_net;") && !MOD.contains("mod net_role;"));
    assert!(!START.contains("prove("), "nothing is proved before the family exists");
}

#[test]
fn a_store_program_is_proved_and_then_bound_with_nothing_kept() {
    let prove = body(PROVE, "prove");
    assert!(prove.contains(
        "Origin::Store => super::super::attest::verify(path, bytes).and_then(|_| bind(path, bytes)),"
    ));
    assert!(prove.contains("Origin::BuiltIn => Ok(bind_built_in(bytes)),"));
    assert!(!PROVE.contains("static KEPT") && !PROVE.contains("fn keep("), "no proof or binding kept");
}

#[test]
fn the_start_proves_after_the_family_exists_and_before_the_guest_runs() {
    let at = |src: &str, needle: &str| src.find(needle).unwrap_or_else(|| panic!("{needle}"));
    let (spawned, started) =
        (at(START, "let pid = mk_foreign_spawn(b\"linux\");"), at(START, "match start(&mut guest, launch)"));
    assert!(spawned < started);
    let start = body(START_GUEST, "start");
    let proved = at(start, "match prove(path, bytes, &launch.origin)");
    let kept = at(start, "super::net::grant::keep(guest.app.policy.encode());");
    let narrowed = at(start, "guest.app = super::boxed::narrow(guest.app);");
    assert!(proved < kept && kept < narrowed, "the grant asks by the program's own policy");
    assert!(narrowed < at(start, "mk_foreign_start("));
}
