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

//! `nslookup` and `ping` in a session: each asks the host, prints the
//! terminal's lines, refuses a name DNS could not carry before asking, and
//! says why nothing was sent off the Direct route.

use crate::kit::{Lines, Script};
use nonos_shell::builtin::dns::{is_host, parse_ipv4, MAX_HOST};
use nonos_shell::builtin::ping::PING_COUNT;
use nonos_shell::host::{Echo, Resolve};
use nonos_shell::session::{Session, NAMES};

const WHY: &str = "the chosen network is the Nym mixnet";

fn run(host: &mut Script, line: &str) -> Vec<String> {
    let mut out = Lines::default();
    Session::default().run_line(line.as_bytes(), &mut out, host);
    out.text()
}

#[test]
fn nslookup_prints_the_address_or_why_there_is_none() {
    let mut h = Script { resolve: Some(Resolve::Addr([93, 184, 216, 34])), ..Script::default() };
    assert_eq!(run(&mut h, "nslookup example.org; echo $?"), ["example.org -> 93.184.216.34", "0"]);
    assert_eq!(h.resolved, [b"example.org".to_vec()]);
    let mut off = Script { resolve: Some(Resolve::Refused(WHY)), ..Script::default() };
    assert_eq!(
        run(&mut off, "nslookup example.org; echo $?"),
        [
            format!("nslookup: {WHY}; a lookup would leave in the clear, so nothing was sent"),
            "1".into()
        ]
    );
    let mut no = Script { resolve: Some(Resolve::NoName), ..Script::default() };
    assert_eq!(
        run(&mut no, "nslookup nowhere.invalid"),
        ["nslookup: no address for nowhere.invalid"]
    );
    let mut dns = Script { resolve: Some(Resolve::NoDns), ..Script::default() };
    assert_eq!(run(&mut dns, "nslookup example.org"), ["nslookup: no DNS server answered"]);
    assert_eq!(run(&mut Script::default(), "nslookup example.org"), ["nslookup: dns unavailable"]);
}

#[test]
fn a_name_dns_could_not_carry_is_never_asked() {
    assert!(is_host(b"db-1.eu.example"));
    assert!(is_host(&[b'a'; MAX_HOST]));
    assert!(!is_host(&[b'a'; MAX_HOST + 1]));
    for bad in [&b""[..], b"a_b", b"a\x1bb", b"exa mple", b"x\x00"] {
        assert!(!is_host(bad), "{bad:?}");
    }
    let mut h = Script { resolve: Some(Resolve::Addr([1, 2, 3, 4])), ..Script::default() };
    let said = run(&mut h, "nslookup a_b");
    assert_eq!(said, ["nslookup: a host name is 1 to 253 letters, digits, dots and hyphens"]);
    let said = run(&mut h, "ping a_b");
    assert_eq!(said, ["ping: a host name is 1 to 253 letters, digits, dots and hyphens"]);
    assert!(h.resolved.is_empty() && h.echoed.is_empty(), "nothing was asked");
}

#[test]
fn ping_sends_its_echoes_and_totals_them() {
    let mut h = Script {
        resolve: Some(Resolve::Addr([10, 0, 2, 2])),
        echoes: vec![Echo::Reply(80), Echo::Timeout, Echo::Reply(40), Echo::Reply(60)],
        ..Script::default()
    };
    assert_eq!(
        run(&mut h, "ping gateway; echo $?"),
        [
            "PING gateway (10.0.2.2)",
            "reply from 10.0.2.2: seq=1 time=80 ms",
            "request timed out: seq=2",
            "reply from 10.0.2.2: seq=3 time=40 ms",
            "reply from 10.0.2.2: seq=4 time=60 ms",
            "--- gateway ping statistics ---",
            "4 transmitted, 3 received, 25% loss",
            "rtt min/avg/max = 40/60/80 ms",
            "0",
        ]
    );
    let sent: Vec<u16> = h.echoed.iter().map(|(_, seq)| *seq).collect();
    assert_eq!(sent, (1..=PING_COUNT).collect::<Vec<_>>());
    assert!(h.echoed.iter().all(|(dst, _)| *dst == [10, 0, 2, 2]));
}

#[test]
fn ping_of_an_address_asks_no_resolver_and_all_lost_fails() {
    let mut h = Script { echoes: vec![Echo::Timeout; 4], ..Script::default() };
    let said = run(&mut h, "ping 10.0.2.2; echo $?");
    assert!(h.resolved.is_empty());
    assert_eq!(said[0], "PING 10.0.2.2 (10.0.2.2)");
    assert_eq!(said[6], "4 transmitted, 0 received, 100% loss");
    assert_eq!(said.last().map(String::as_str), Some("1"));
    assert_eq!(parse_ipv4(b"10.0.2.2"), Some([10, 0, 2, 2]));
    for bad in [&b"256.0.0.1"[..], b"1.2.3", b"1.2.3.4.5", b"1..2.3", b"01234.1.1.1", b""] {
        assert_eq!(parse_ipv4(bad), None, "{bad:?}");
    }
}

#[test]
fn off_the_direct_route_ping_sends_nothing_and_says_why() {
    let line =
        format!("ping: {WHY}; ICMP cannot cross it and would leave directly, so nothing was sent");
    let mut h = Script { echoes: vec![Echo::Refused(WHY)], ..Script::default() };
    assert_eq!(run(&mut h, "ping 10.0.2.2; echo $?"), [line.clone(), "1".into()]);
    assert_eq!(h.echoed.len(), 1, "the first refusal stops it");
    let mut named = Script { resolve: Some(Resolve::Refused(WHY)), ..Script::default() };
    assert_eq!(
        run(&mut named, "ping example.org"),
        [format!("ping: {WHY}; a lookup would leave in the clear, so nothing was sent")]
    );
    assert!(named.echoed.is_empty(), "no echo without an address");
}

#[test]
fn a_fault_that_will_not_fix_itself_is_said_once() {
    let mut h = Script { echoes: vec![Echo::Reply(5), Echo::NoRoute], ..Script::default() };
    let said = run(&mut h, "ping 10.0.2.2");
    assert_eq!(said[1], "reply from 10.0.2.2: seq=1 time=5 ms");
    assert_eq!(said[2], "no route to host");
    assert_eq!(said[4], "1 transmitted, 1 received, 0% loss", "the stopped echo is not counted");
    assert_eq!(h.echoed.len(), 2);
    let mut none = Script::default();
    assert_eq!(run(&mut none, "ping 10.0.2.2"), ["ping: net unavailable"]);
    assert_eq!(run(&mut none, "ping"), ["usage: ping <host>"]);
}

#[test]
fn help_and_type_know_both() {
    for name in ["nslookup", "ping"] {
        assert!(NAMES.contains(&name.as_bytes()), "{name}");
    }
}
