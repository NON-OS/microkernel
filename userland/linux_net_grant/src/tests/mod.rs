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

//! The grant rule held to the facts a test chooses: no NET_ANON gets none, a
//! family the kernel proved neither the policy nor the program in gets none,
//! a box is read from the kernel's argv alone and carries its destinations,
//! and a grant serves only the family it was made for.

extern crate std;

use alloc::vec::Vec;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet};

use nonos_linux_policy::mask::{FILES_HOME, NET_ANON};
use nonos_linux_policy::vouch::vouched_hash;
use nonos_linux_policy::{unlisted, LinuxAppPolicy};

use crate::{by_box, by_policy, decide, names_destinations, Ask, Facts, Grant, Grants, Refusal};
use crate::{LOCAL, MAX_ROWS, PUBLISHER};

const PID: u32 = 0x7F;
const PROGRAM: [u8; 32] = [7; 32];

/// The kernel as a test has it: each pid's live family by generation, what
/// each family recorded, who holds Network, and the argv it set.
#[derive(Default)]
struct Kernel {
    generation: RefCell<BTreeMap<u32, u64>>,
    /// Each family's measurements, with the kinds that verified them.
    measured: RefCell<BTreeMap<(u64, [u8; 32]), u8>>,
    network: BTreeSet<u32>,
    argv: Option<Vec<u8>>,
    /// When set, every generation asked for is a new one: a family that
    /// ends and is replaced between two asks.
    churn: Cell<bool>,
}

impl Facts for Kernel {
    fn holds_network(&self, pid: u32) -> bool {
        self.network.contains(&pid)
    }
    fn holds(&self, pid: u32, m: &[u8; 32], kinds: u8) -> bool {
        // The family whose measurements are asked about, as it was when its
        // generation was last given: a family's own record does not churn.
        let g = self.generation.borrow().get(&pid).copied().unwrap_or(0);
        let g = if self.churn.get() { g.saturating_sub(1) } else { g };
        g != 0 && self.measured.borrow().get(&(g, *m)).is_some_and(|k| k & kinds != 0)
    }
    fn generation(&self, pid: u32) -> u64 {
        let mut gens = self.generation.borrow_mut();
        let g = gens.get(&pid).copied().unwrap_or(0);
        if self.churn.get() && g != 0 {
            gens.insert(pid, g + 1);
        }
        g
    }
    fn argv(&self, _pid: u32) -> Option<Vec<u8>> {
        self.argv.clone()
    }
}

impl Kernel {
    /// `pid`'s family is now generation `g`, which recorded `measured` under
    /// local trailers, as an install on this machine leaves them.
    fn family(&self, pid: u32, g: u64, measured: &[[u8; 32]]) {
        let local: Vec<([u8; 32], u8)> = measured.iter().map(|m| (*m, LOCAL)).collect();
        self.family_by(pid, g, &local);
    }

    /// `pid`'s family is now generation `g`, which recorded each measurement
    /// under the kind beside it, as the kernel keeps them (seal/kind.rs).
    fn family_by(&self, pid: u32, g: u64, measured: &[([u8; 32], u8)]) {
        self.generation.borrow_mut().insert(pid, g);
        measured.iter().for_each(|(m, kind)| {
            *self.measured.borrow_mut().entry((g, *m)).or_insert(0) |= kind;
        });
    }
}

fn policy(mask: u64) -> [u8; 256] {
    LinuxAppPolicy { mask, ..unlisted(PROGRAM) }.encode()
}

/// A family of generation 5 that proved `PROGRAM` and had `policy` verified.
fn family(policy: &[u8]) -> Kernel {
    let k = Kernel::default();
    k.family(PID, 5, &[PROGRAM, vouched_hash(policy)]);
    k
}

fn boxed(word: &[u8]) -> Kernel {
    let k = Kernel { argv: Some([&b"linux\0--box\0"[..], word, b"\0curl"].concat()), ..Default::default() };
    k.family(PID, 9, &[]);
    k
}

#[test]
fn a_policy_with_net_anon_vouched_and_proved_in_the_family_is_granted() {
    let p = policy(NET_ANON | FILES_HOME);
    assert_eq!(by_policy(PID, &p, &family(&p)), Ok(Grant::Policy { program: PROGRAM }));
    assert_eq!(decide(PID, Ask::Policy(&p), &family(&p)), Ok((Grant::Policy { program: PROGRAM }, 5)));
}

#[test]
fn a_program_without_net_anon_gets_no_grant() {
    let p = policy(FILES_HOME);
    assert_eq!(by_policy(PID, &p, &family(&p)), Err(Refusal::NoNetAnon));
}

#[test]
fn a_family_whose_measurements_lack_the_policy_or_the_program_gets_no_grant() {
    let p = policy(NET_ANON);
    let k = Kernel::default();
    k.family(PID, 5, &[PROGRAM]);
    assert_eq!(by_policy(PID, &p, &k), Err(Refusal::PolicyNotVouched));
    let k = Kernel::default();
    k.family(PID, 5, &[vouched_hash(&p)]);
    assert_eq!(by_policy(PID, &p, &k), Err(Refusal::ProgramNotProved));
    let other = policy(NET_ANON | FILES_HOME);
    assert_eq!(by_policy(PID, &other, &family(&p)), Err(Refusal::PolicyNotVouched), "another policy");
    assert_eq!(by_policy(PID + 1, &p, &family(&p)), Err(Refusal::PolicyNotVouched), "another family");
    let k = family(&p);
    k.family(PID, 6, &[]);
    assert_eq!(by_policy(PID, &p, &k), Err(Refusal::PolicyNotVouched), "the next family");
}

/// A blob the kernel measured as a program (MkLocalVerify or
/// MkCapsuleVerify over its own bytes) that parses as a policy granting
/// NET_ANON and naming a program the family proved: measured under its plain
/// BLAKE3, never under the policy domain, so it vouches for nothing.
#[test]
fn a_program_shaped_attested_blob_that_parses_as_a_policy_is_refused() {
    let p = policy(NET_ANON | FILES_HOME);
    let k = Kernel::default();
    k.family(PID, 5, &[PROGRAM, *blake3::hash(&p).as_bytes()]);
    assert_eq!(by_policy(PID, &p, &k), Err(Refusal::PolicyNotVouched));
    assert_eq!(decide(PID, Ask::Policy(&p), &k), Err(Refusal::PolicyNotVouched));
    let k = Kernel::default();
    k.family(PID, 5, &[PROGRAM, *blake3::hash(&p).as_bytes(), vouched_hash(&p)]);
    assert_eq!(by_policy(PID, &p, &k), Ok(Grant::Policy { program: PROGRAM }), "vouched as a policy");
}

/// The policy domain's hash recorded under a publisher's chain: a payload a
/// certified publisher signed that is exactly the domain and a policy, which
/// a compromised personality had the kernel verify. The kernel says it was
/// never a local trailer, so it vouches for nothing; the program may still be
/// a publisher's.
#[test]
fn a_publisher_verified_blob_that_is_the_domain_and_a_policy_is_refused_as_a_policy() {
    let p = policy(NET_ANON | FILES_HOME);
    let k = Kernel::default();
    k.family_by(PID, 5, &[(PROGRAM, LOCAL), (vouched_hash(&p), PUBLISHER)]);
    assert_eq!(by_policy(PID, &p, &k), Err(Refusal::PolicyNotVouched));
    assert_eq!(decide(PID, Ask::Policy(&p), &k), Err(Refusal::PolicyNotVouched));
    let k = Kernel::default();
    k.family_by(PID, 5, &[(PROGRAM, PUBLISHER), (vouched_hash(&p), LOCAL)]);
    assert_eq!(by_policy(PID, &p, &k), Ok(Grant::Policy { program: PROGRAM }), "a publisher's program");
    let k = Kernel::default();
    k.family_by(PID, 5, &[(PROGRAM, LOCAL), (vouched_hash(&p), PUBLISHER | LOCAL)]);
    assert_eq!(by_policy(PID, &p, &k), Ok(Grant::Policy { program: PROGRAM }), "vouched here too");
}

#[test]
fn bytes_that_are_not_a_policy_get_no_grant() {
    let p = policy(NET_ANON);
    let k = family(&p);
    assert_eq!(by_policy(PID, &p[..255], &k), Err(Refusal::NotAPolicy));
    let mut bad = p;
    bad[0] ^= 1;
    assert_eq!(by_policy(PID, &bad, &k), Err(Refusal::NotAPolicy));
}

#[test]
fn no_family_or_a_family_that_changes_while_deciding_gets_no_grant() {
    let p = policy(NET_ANON);
    assert_eq!(decide(PID, Ask::Policy(&p), &Kernel::default()), Err(Refusal::NoFamily));
    let k = family(&p);
    k.churn.set(true);
    assert_eq!(decide(PID, Ask::Policy(&p), &k), Err(Refusal::NoFamily));
}

#[test]
fn a_box_is_read_from_the_kernels_argv_and_carries_its_destinations() {
    let Ok(Grant::Boxed { dests }) = by_box(PID, &boxed(b"allow:/tmp/o,net:example.com:443,net:10.0.0.1:80")) else {
        panic!("a box with destinations is granted")
    };
    assert!(!dests.is_empty());
    assert_eq!(by_box(PID, &boxed(b"allow:/tmp/o")), Err(Refusal::NoBox), "no destination");
    assert_eq!(by_box(PID, &boxed(b"strict")), Err(Refusal::NoBox));
    let mut k = boxed(b"allow:net:x:1");
    k.argv = Some(b"linux\0curl\0--box\0allow:net:x:1".to_vec());
    assert_eq!(by_box(PID, &k), Err(Refusal::NoBox), "a --box later is the program's");
    k.argv = None;
    assert_eq!(by_box(PID, &k), Err(Refusal::NoBox));
    assert!(names_destinations(b"linux\0--box-key=ab\0--box\0allow:net:a:1"));
}

#[test]
fn a_box_grant_reaches_its_destinations_and_nothing_else() {
    let k = boxed(b"allow:net:Example.com:443,net:10.0.0.1:80");
    let (grant, g) = decide(PID, Ask::Box, &k).unwrap();
    let mut grants = Grants::new();
    grants.insert(PID, g, grant).unwrap();
    assert!(grants.may_reach(PID, b"example.com", 443, &k));
    assert!(grants.may_reach(PID, b"EXAMPLE.COM.", 443, &k), "as the personality matches names");
    assert!(grants.may_reach(PID, b"10.0.0.1", 80, &k));
    assert!(!grants.may_reach(PID, b"example.com", 80, &k), "another port");
    assert!(!grants.may_reach(PID, b"10.0.0.2", 80, &k), "another address");
    assert!(!grants.may_reach(PID, b"evil.example", 443, &k), "another name");
}

#[test]
fn a_policy_grant_reaches_anywhere_over_the_network_it_is_carried_on() {
    let p = policy(NET_ANON);
    let k = family(&p);
    let mut grants = Grants::new();
    let (grant, g) = decide(PID, Ask::Policy(&p), &k).unwrap();
    grants.insert(PID, g, grant).unwrap();
    assert!(grants.may_reach(PID, b"anything.example", 1, &k));
}

#[test]
fn a_sender_without_network_or_a_grant_is_not_served() {
    let mut grants = Grants::new();
    let mut k = Kernel::default();
    assert!(!grants.admits(PID, &k));
    assert!(!grants.may_reach(PID, b"example.com", 443, &k));
    k.network.insert(PID);
    assert!(grants.admits(PID, &k), "a Network holder is served as always");
    assert!(grants.may_reach(PID, b"example.com", 443, &k));
}

#[test]
fn the_next_family_under_the_same_pid_is_refused_its_first_stream() {
    let p = policy(NET_ANON);
    let k = family(&p);
    let mut grants = Grants::new();
    let (grant, g) = decide(PID, Ask::Policy(&p), &k).unwrap();
    grants.insert(PID, g, grant).unwrap();
    assert!(grants.admits(PID, &k));
    // The family is sealed and the next one reserved under the same pid,
    // running a program granted nothing: no window, no recheck interval.
    k.family(PID, 6, &[]);
    assert!(!grants.may_reach(PID, b"example.com", 443, &k), "its first stream");
    assert!(!grants.admits(PID, &k));
    assert_eq!(grants.pids().count(), 0, "the row went with its family");
}

#[test]
fn a_grant_dies_with_its_family() {
    let k = boxed(b"allow:net:a.example:1");
    let mut grants = Grants::new();
    let (grant, g) = decide(PID, Ask::Box, &k).unwrap();
    grants.insert(PID, g, grant).unwrap();
    k.generation.borrow_mut().remove(&PID);
    assert!(!grants.admits(PID, &k));
}

#[test]
fn reaping_drops_the_rows_of_ended_families_only() {
    let k = Kernel::default();
    k.family(PID, 3, &[]);
    k.family(PID + 1, 4, &[]);
    let mut grants = Grants::new();
    grants.insert(PID, 3, Grant::Policy { program: PROGRAM }).unwrap();
    grants.insert(PID + 1, 2, Grant::Policy { program: PROGRAM }).unwrap();
    grants.reap(&k);
    assert_eq!(grants.pids().collect::<Vec<_>>(), [PID]);
}

#[test]
fn a_full_table_refuses_a_new_grant_and_keeps_its_rows() {
    let mut grants = Grants::new();
    let g = || Grant::Policy { program: PROGRAM };
    for pid in 0..MAX_ROWS as u32 {
        grants.insert(pid, 1, g()).unwrap();
    }
    assert_eq!(grants.insert(9999, 1, g()), Err(Refusal::Full));
    assert_eq!(grants.insert(3, 2, g()), Ok(()), "a row asked again is replaced");
    assert_eq!(grants.pids().count(), MAX_ROWS);
}

/// net.anon serves the ops the personality sends: its protocol names them by
/// number, and these are the numbers this crate gives the personality.
#[test]
fn net_anon_names_the_grant_ops_by_the_same_numbers() {
    use crate::wire::{ANON_OP_GRANT_BOX, ANON_OP_GRANT_POLICY};
    let ops = include_str!("../../../capsule_net_anon/src/protocol/ops.rs");
    assert!(ops.contains(&std::format!("pub const OP_GRANT_POLICY: u16 = {ANON_OP_GRANT_POLICY};")));
    assert!(ops.contains(&std::format!("pub const OP_GRANT_BOX: u16 = {ANON_OP_GRANT_BOX};")));
}

const ANON_GATE: &str = include_str!("../../../capsule_net_anon/src/server/grant.rs");
const ANON_RUNNER: &str = include_str!("../../../capsule_net_anon/src/server/runner.rs");
const SOCKS_GATE: &str = include_str!("../../../capsule_socks5/src/server/grant.rs");
const SOCKS_RUN: &str = include_str!("../../../capsule_socks5/src/server/run.rs");
const SOCKS_FEED: &str = include_str!("../../../capsule_socks5/src/server/feed.rs");

fn at(src: &str, needle: &str) -> usize {
    src.find(needle).unwrap_or_else(|| panic!("{needle} is gone"))
}

/// Both fronts ask the grant before anything else is served, and open a
/// stream only where the grant reaches.
#[test]
fn both_fronts_gate_every_request_and_every_stream_open() {
    assert!(at(ANON_RUNNER, "if gate(&mut grants, sender, frame, &mut tx) {") < at(ANON_RUNNER, "if answer(&mut front"));
    assert!(ANON_GATE.contains("const GRANTED_OPS: [u16; 4] = [OP_OPEN_STREAM, OP_SEND, OP_RECV, OP_CLOSE_STREAM];"));
    assert!(ANON_GATE.contains("grants.may_reach(sender, host, port, &facts)"));
    assert!(at(SOCKS_RUN, "super::grant::gate(sender, &rx[..n as usize])") < at(SOCKS_RUN, "match ask(&rx[..n as usize])"));
    let guard = at(SOCKS_FEED, "super::grant::may_open(super::who::pid_of(pid), &dest)");
    assert!(guard < at(SOCKS_FEED, "server.manager.open(pid)") && guard < at(SOCKS_FEED, "open_tunnel(id, &dest)"));
}

/// The argv a box grant is read from is never logged or kept by a front.
#[test]
fn no_front_logs_or_keeps_the_argv() {
    for gate in [ANON_GATE, SOCKS_GATE] {
        for said in ["mk_debug", "println", "say(", "trace", "argv"] {
            assert!(!gate.lines().filter(|l| !l.trim_start().starts_with("//")).any(|l| l.contains(said)), "{said}");
        }
    }
    let by_box = include_str!("../by_box.rs");
    assert!(by_box.contains("Ok(Grant::Boxed { dests })"), "only the destinations are kept");
}

const ANON_LINE: &str = include_str!("../../../capsule_net_anon/src/server/grant_line.rs");
const SOCKS_LINE: &str = include_str!("../../../capsule_socks5/src/server/grant_line.rs");

/// Each front says a grant decision on the serial log by its code alone:
/// the line's one function takes a u8 and nothing the decision read.
#[test]
fn each_front_says_a_decision_by_its_code_alone() {
    for (line, gate) in [(ANON_LINE, ANON_GATE), (SOCKS_LINE, SOCKS_GATE)] {
        let code: Vec<&str> = line
            .lines()
            .filter(|l| !l.trim_start().starts_with("//") && !l.trim().is_empty())
            .collect();
        assert_eq!(code.iter().filter(|l| l.contains("fn ")).count(), 1, "one function");
        assert!(code.iter().any(|l| l.contains("pub fn said(code: u8) {")));
        for word in ["argv", "host", "dest", "policy", "measure", "program"] {
            assert!(!code.iter().any(|l| l.contains(word)), "{word}");
        }
        assert_eq!(gate.matches("super::grant_line::said(").count(), 1, "one decision line");
    }
    assert!(SOCKS_LINE.contains("super::trace::step(b\"grant\", u64::from(code));"));
    assert!(ANON_LINE.contains("crate::trace::say_num(b\"grant\", u64::from(code));"));
}
