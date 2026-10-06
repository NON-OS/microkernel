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

use nonos_route_proof::{Network, RouteVerdict, Stage, Stale};

use crate::proof_board::census::{
    census, Admitted, Proc, AUTHORITY_PUBLISHER, CAP_NETWORK, STARTING_MS,
};
use crate::proof_board::session::{
    admitted_mark, boot_mark, headline, proven_mark, route_mark, Boot, Headline, Mark,
};
use crate::proof_board::words;

const RUNNING: u8 = 2;
const SLEEPING: u8 = 3;
const NEW: u8 = 0;
const ZOMBIE: u8 = 5;

fn p(pid: u32, state: u8, caps: u64, name: &'static [u8]) -> Proc<'static> {
    Proc { pid, state, uptime_ms: 60_000, caps, name }
}

fn a(pid: u32, caps: u64, authority: u8) -> Admitted {
    Admitted { pid, caps, authority }
}

#[test]
fn a_machine_where_every_holder_was_admitted_counts_clean() {
    let procs = [
        p(1, SLEEPING, 0x3FF, b"init"),
        p(2, RUNNING, 0x19, b"vfs"),
        p(3, SLEEPING, 0x1D, b"net.anon"),
        p(4, SLEEPING, 0, b"foreign:busybox"),
    ];
    let reg = [a(2, 0x19, 0), a(3, 0x1D, 0)];
    let c = census(&procs, &reg);
    assert_eq!((c.running, c.admitted, c.unadmitted), (4, 2, 0));
    assert_eq!((c.kernel, c.sandboxed, c.network), (1, 1, 1));
    assert_eq!(admitted_mark(Some(&c)), Mark::Holds);
    assert_eq!(proven_mark(Some(&c)), Mark::Holds);
}

#[test]
fn a_process_holding_authority_with_no_record_breaks_the_claim_and_is_named() {
    let procs = [p(2, RUNNING, 0x19, b"vfs"), p(9, RUNNING, 0x4, b"rogue")];
    let c = census(&procs, &[a(2, 0x19, 0)]);
    assert_eq!(c.unadmitted, 1);
    assert_eq!(c.first_unadmitted, Some(9));
    assert_eq!(admitted_mark(Some(&c)), Mark::Broken);
}

#[test]
fn a_capsule_mid_spawn_is_neither_passed_nor_failed() {
    let young = Proc { pid: 7, state: RUNNING, uptime_ms: STARTING_MS - 1, caps: 0x19, name: b"app" };
    let c = census(&[p(2, RUNNING, 0x19, b"vfs"), young], &[a(2, 0x19, 0)]);
    assert_eq!((c.starting, c.unadmitted), (1, 0));
    let old = Proc { uptime_ms: STARTING_MS, ..young };
    assert_eq!(census(&[old], &[]).unadmitted, 1);
}

#[test]
fn new_and_exited_processes_are_not_counted() {
    let procs = [p(5, NEW, 0x19, b"app"), p(6, ZOMBIE, 0x19, b"gone")];
    let c = census(&procs, &[]);
    assert_eq!(c.running, 0);
    assert_eq!(c.unadmitted, 0);
}

#[test]
fn only_init_by_name_is_the_kernels_own() {
    let c = census(&[p(1, RUNNING, 0x3FF, b"initx")], &[]);
    assert_eq!((c.kernel, c.unadmitted), (0, 1));
}

#[test]
fn a_capsule_on_a_publisher_signature_alone_breaks_proven() {
    let c = census(
        &[p(2, RUNNING, 0x19, b"vfs"), p(3, RUNNING, 0x19, b"app")],
        &[a(2, 0x19, 0), a(3, 0x19, AUTHORITY_PUBLISHER)],
    );
    assert_eq!((c.vendor, c.signed_only), (1, 1));
    assert_eq!(admitted_mark(Some(&c)), Mark::Holds);
    assert_eq!(proven_mark(Some(&c)), Mark::Broken);
}

#[test]
fn locally_enrolled_roots_count_as_proven() {
    let c = census(&[p(2, RUNNING, 0x19, b"tool")], &[a(2, 0x19, 7)]);
    assert_eq!(c.enrolled, 1);
    assert_eq!(proven_mark(Some(&c)), Mark::Holds);
}

#[test]
fn network_holders_are_counted_from_the_registry_mask() {
    let c = census(
        &[p(2, RUNNING, 0, b"a"), p(3, RUNNING, 0, b"b")],
        &[a(2, CAP_NETWORK | 0x18, 0), a(3, 0x18, 0)],
    );
    assert_eq!(c.network, 1);
}

#[test]
fn nothing_readable_proves_nothing() {
    assert_eq!(admitted_mark(None), Mark::Unknown);
    assert_eq!(proven_mark(None), Mark::Unknown);
    let empty = census(&[], &[]);
    assert_eq!(admitted_mark(Some(&empty)), Mark::Unknown);
}

fn boot(s: Mark, a: Mark, p: Mark) -> Mark {
    boot_mark(&Boot { signature: s, attestation: a, proof: p })
}

#[test]
fn the_boot_chain_holds_only_when_all_three_hold() {
    use Mark::*;
    for s in [Holds, Broken, Unknown] {
        for at in [Holds, Broken, Unknown] {
            for pr in [Holds, Broken, Unknown] {
                let m = boot(s, at, pr);
                let any_broken = [s, at, pr].contains(&Broken);
                let all_hold = [s, at, pr].iter().all(|x| *x == Holds);
                let want = if any_broken { Broken } else if all_hold { Holds } else { Unknown };
                assert_eq!(m, want);
            }
        }
    }
}

#[test]
fn the_headline_never_claims_more_than_every_check_under_it() {
    use Mark::*;
    let routes = [
        RouteVerdict::Anonymous(Network::Nym),
        RouteVerdict::Anonymous(Network::Anyone),
        RouteVerdict::Exposed,
        RouteVerdict::NotEstablished(Network::Nym, Stale::NoReport),
        RouteVerdict::NotEstablished(Network::Anyone, Stale::Stage(Stage::Joining)),
        RouteVerdict::Unknown,
    ];
    for b in [Holds, Broken, Unknown] {
        for ad in [Holds, Broken, Unknown] {
            for pr in [Holds, Broken, Unknown] {
                for r in routes {
                    let h = headline(b, ad, pr, r);
                    let attested = b == Holds && ad == Holds && pr == Holds;
                    let broken = [b, ad, pr].contains(&Broken);
                    match h {
                        Headline::AttestedAndAnonymous(_) => {
                            assert!(attested && route_mark(r) == Holds, "{h:?} {r:?}")
                        }
                        Headline::AttestedNotAnonymous => {
                            assert!(attested && r == RouteVerdict::Exposed)
                        }
                        Headline::AttestedRouteDown => assert!(attested),
                        Headline::NotAttested => assert!(broken),
                        Headline::Unknown => assert!(!broken),
                    }
                    if broken {
                        assert_eq!(h, Headline::NotAttested, "a broken check must lead");
                    }
                }
            }
        }
    }
}

#[test]
fn the_headline_words_say_exactly_the_verdict() {
    let anon = words::headline(Headline::AttestedAndAnonymous(Network::Nym));
    assert!(contains(anon, b"anonymous") && contains(anon, b"Nym"));
    for h in [
        Headline::AttestedNotAnonymous,
        Headline::AttestedRouteDown,
        Headline::NotAttested,
        Headline::Unknown,
    ] {
        let w = words::headline(h);
        assert!(!w.starts_with(b"Attested, and anonymous"), "{h:?} reads as a pass");
    }
    assert!(contains(words::headline(Headline::AttestedNotAnonymous), b"not anonymous"));
    assert!(contains(words::headline(Headline::NotAttested), b"Not attested"));
}

#[test]
fn counts_and_durations_are_written_without_overflow() {
    let mut b = [0u8; 48];
    assert_eq!(words::of(3, 3, &mut b), b"3 of 3");
    let mut b = [0u8; 48];
    assert_eq!(words::of(u64::MAX, u64::MAX, &mut b), b"18446744073709551615 of 18446744073709551615");
    for (ms, want) in [
        (0u64, &b"0 s"[..]),
        (59_999, b"59 s"),
        (60_000, b"1 min"),
        (3_599_999, b"59 min"),
        (3_600_000, b"1 h 0 min"),
        (7_440_000, b"2 h 4 min"),
        (86_400_000, b"1 d 0 h"),
    ] {
        let mut b = [0u8; 48];
        assert_eq!(words::duration(ms, &mut b), want, "{ms}");
    }
    let mut b = [0u8; 48];
    assert!(!words::duration(u64::MAX, &mut b).is_empty());
}

fn contains(hay: &[u8], needle: &[u8]) -> bool {
    hay.windows(needle.len()).any(|w| w == needle)
}

#[test]
fn every_reason_and_stage_has_its_own_words() {
    let reasons = [
        Stale::NoReport,
        Stale::Old,
        Stale::Stage(Stage::Cold),
        Stale::DirectoryUnsigned,
        Stale::DirectoryExpired,
        Stale::NoOpenRoute,
        Stale::HopUnauthenticated,
    ];
    for (i, x) in reasons.iter().enumerate() {
        assert!(!words::why(*x).is_empty());
        for y in &reasons[i + 1..] {
            assert_ne!(words::why(*x), words::why(*y), "{x:?} and {y:?} read the same");
        }
    }
    let stages = [Stage::Cold, Stage::Bootstrapping, Stage::Joining, Stage::Ready, Stage::Failed];
    for (i, x) in stages.iter().enumerate() {
        for y in &stages[i + 1..] {
            assert_ne!(words::stage(*x), words::stage(*y));
        }
    }
    assert_eq!(words::stage(Stage::Ready), b"up");
}

#[test]
fn the_route_evidence_names_the_network_or_the_reason() {
    assert_eq!(words::route_evidence(RouteVerdict::Anonymous(Network::Anyone)), words::network(Network::Anyone));
    assert_eq!(words::route_evidence(RouteVerdict::Exposed), b"direct");
    let r = RouteVerdict::NotEstablished(Network::Nym, Stale::DirectoryExpired);
    assert_eq!(words::route_evidence(r), words::why(Stale::DirectoryExpired));
    assert_ne!(words::network(Network::Nym), words::network(Network::Anyone));
}

#[test]
fn the_directory_and_hop_lines_carry_the_reports_numbers() {
    let r = nonos_route_proof::RouteReport {
        network: Network::Anyone,
        stage: Stage::Ready,
        signatures_verified: 6,
        signatures_required: 5,
        nodes: 4_812,
        valid_for_ms: 1,
        routes_open: 2,
        hops_authenticated: 2,
        hops: 3,
        surbs: 0,
        cover_traffic: false,
        reason: 0,
    };
    let mut b = [0u8; 48];
    assert_eq!(words::directory(&r, &mut b), b"6 of 5 signatures, 4812 nodes");
    let mut b = [0u8; 48];
    assert_eq!(words::hops(&r, &mut b), b"2 of 3 hops authenticated");
    let big = nonos_route_proof::RouteReport { nodes: u32::MAX, signatures_verified: 255, signatures_required: 255, ..r };
    let mut b = [0u8; 48];
    assert!(words::directory(&big, &mut b).len() <= 48, "a long line is cut, never overflowed");
}
