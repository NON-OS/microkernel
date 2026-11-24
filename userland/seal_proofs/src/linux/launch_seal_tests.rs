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

//! I-SEAL: a store run's own program is in its family's measurements, once,
//! and so is the policy it was bound to: the two facts an anonymity front
//! reads back before it grants the family the network.
//!
//! The kernel keeps a family's measurement list M from the family's first
//! guest on (seal/open.rs reserve, called by spawn_guest.rs), and a proof the
//! kernel accepts before then is absorbed by no family (seal/absorb.rs
//! measured, a no-op without one). The personality's launch is replayed here
//! in the order its source runs it: the guest's spawn, then
//! start_guest::start's prove, the shipped prove.rs itself. Until
//! 2026-11-23 the program was proved before the spawn (run_net::early) and
//! the start took that proof as kept, which left M without the program.
//! run_net::early now binds without a proof (start_guest/bind_role.rs).

use crate::foreign::seal::family::Family;
use crate::foreign::seal::kind::LOCAL;
use crate::foreign::seal::table::{with_family, LIVE};

use super::origin::Origin;
use super::app::POLICY;
use super::start_guest::prove;

/// Its own supervisor, apart from every other test's families.
pub const SUPERVISOR: u32 = 0x5EA1;

const START: &str = include_str!("../../../capsule_linux/src/linux/start.rs");
const RUN_NET: &str = include_str!("../../../capsule_linux/src/linux/run_net.rs");
const ABSORB: &str = include_str!("../../../../src/process/foreign/seal/absorb.rs");
const OPEN: &str = include_str!("../../../../src/process/foreign/seal/open.rs");
const SPAWN_GUEST: &str = include_str!("../../../../src/process/foreign/spawn_guest.rs");

/// seal/absorb.rs `measured`, for the caller `sup`.
pub fn kernel_measured(sup: u32, m: &[u8; 32]) {
    with_family(sup, |f| f.measured(m, LOCAL));
}

/// seal/open.rs `reserve` at a supervisor's first guest: a fresh family.
fn kernel_first_guest(sup: u32) {
    let mut live = LIVE.lock();
    if !live.iter().any(|f| f.sup == sup) {
        let mut f = Family::fresh(sup, ([0xAB; 32], 0x1_0400_1979, 0), [9; 32], [5, 6, 7, 8]);
        f.enrolled = true;
        live.push(f);
    }
}

fn end_family(sup: u32) {
    LIVE.lock().retain(|f| f.sup != sup);
}

/// What the family recorded of `program`, and of its policy, after one store
/// launch of it.
fn launch(program: &[u8]) -> ((usize, usize), u32) {
    let path: &[u8] = b"/usr/bin/curl";
    // start.rs: mk_foreign_spawn, where the kernel reserves the family
    kernel_first_guest(SUPERVISOR);
    // start_guest::start
    prove(path, program, &Origin::Store).expect("the program proves");
    let (m, p) = (*blake3::hash(program).as_bytes(), *blake3::hash(POLICY).as_bytes());
    let count = |f: &Family, x: [u8; 32]| f.list.iter().filter(|y| **y == x).count();
    let seen = with_family(SUPERVISOR, |f| ((count(f, m), count(f, p)), f.n_meas));
    end_family(SUPERVISOR);
    seen.expect("the family is live")
}

#[test]
fn a_store_runs_program_is_measured_into_its_family_exactly_once() {
    let program = b"\x7fELF a store program".repeat(64);
    assert_eq!(launch(&program), ((1, 1), 2), "M holds the program and its policy, once each");
}

#[test]
fn the_replay_is_the_order_the_personality_and_the_kernel_run() {
    let at = |src: &str, needle: &str| src.find(needle).unwrap_or_else(|| panic!("{needle}"));
    let early = at(START, "super::run_net::early(&launch);");
    assert!(early < at(START, "let pid = mk_foreign_spawn(b\"linux\");"));
    assert!(!RUN_NET.contains("start_guest::prove("), "nothing is proved before the family");
    assert!(at(START, "let pid = mk_foreign_spawn(b\"linux\");") < at(START, "start(&mut guest"));
    let measured = &ABSORB[at(ABSORB, "pub(crate) fn measured(")..];
    assert!(measured.contains("with_family(caller, |f| f.measured(measurement, kind));"));
    assert!(OPEN.contains("let mut family = Family::fresh(supervisor,"));
    assert!(OPEN.contains("family.generation = super::generation::next();\n    live.push(family);"));
    assert!(SPAWN_GUEST.contains("super::seal::reserve(supervisor)?;"));
}

#[test]
fn the_built_in_program_is_the_manifests_and_asks_the_kernel_nothing() {
    kernel_first_guest(SUPERVISOR + 1);
    let app = prove(b"/bin/busybox", b"busybox", &Origin::BuiltIn).expect("built in");
    assert!(app.built_in);
    let n = with_family(SUPERVISOR + 1, |f| f.n_meas);
    LIVE.lock().retain(|f| f.sup != SUPERVISOR + 1);
    assert_eq!(n, Some(0));
}
