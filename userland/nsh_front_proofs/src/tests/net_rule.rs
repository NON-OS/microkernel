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

//! The frames behind a session's `nslookup` and `ping`: every answer's
//! length is checked before it is read, only the reply to this echo counts,
//! and every errno comes to the answer the shell prints.

use nonos_shell::host::{Echo, Resolve};

use crate::net_rule::{
    answer, checksum, echo_request, header, is_reply, resolved, sent, Sent, DNS_NOT_DIRECT,
    ECHO_ID, ECHO_LEN, HDR, NET_CORE_REFUSED, PROTO_ICMP,
};

const ASK: &str = include_str!("../../../capsule_nsh_front/src/net_ask.rs");
const HOST: &str = include_str!("../../../capsule_nsh_front/src/shell_host.rs");
const DNS: &str = include_str!("../../../capsule_net_core/src/protocol/dns.rs");
const IP: &str = include_str!("../../../capsule_net_core/src/protocol/ip.rs");

fn frame(errno: u16, body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; HDR];
    f[8..10].copy_from_slice(&errno.to_le_bytes());
    f[16..20].copy_from_slice(&(body.len() as u32).to_le_bytes());
    f.extend_from_slice(body);
    f
}

#[test]
fn an_answer_is_read_only_as_far_as_it_arrived() {
    let f = frame(0, &[1, 2, 3, 4]);
    assert_eq!(answer(&f, f.len() as i64), Some((0, &[1u8, 2, 3, 4][..])));
    assert_eq!(answer(&f, -1), None, "the call failed");
    assert_eq!(answer(&f, (HDR - 1) as i64), None, "shorter than a header");
    assert_eq!(answer(&f, (HDR + 3) as i64), None, "a body longer than arrived");
    assert_eq!(answer(&f, (f.len() + 1) as i64), None, "more than the buffer");
    let mut lying = frame(0, &[]);
    lying[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(answer(&lying, lying.len() as i64), None);
}

#[test]
fn a_lookup_comes_to_an_address_or_why_not() {
    assert_eq!(resolved(Some((0, &[93, 184, 216, 34]))), Resolve::Addr([93, 184, 216, 34]));
    assert_eq!(resolved(Some((0, &[1, 2, 3]))), Resolve::NoName, "a short address");
    assert_eq!(resolved(Some((DNS_NOT_DIRECT, &[]))), Resolve::Refused(NET_CORE_REFUSED));
    assert_eq!(resolved(None), Resolve::NoDns, "net.dns did not answer");
    assert_eq!(resolved(Some((6, &[]))), Resolve::NoDns, "no server answered");
    assert_eq!(resolved(Some((11, &[]))), Resolve::NoDns, "no lease");
    assert_eq!(resolved(Some((10, &[]))), Resolve::NoName, "answered without one");
    assert!(DNS.contains("pub const E_NOT_DIRECT: u16 = 12;"));
    assert!(DNS.contains("pub const E_TIMEOUT: u16 = 6;"));
    assert!(DNS.contains("pub const E_NO_LEASE: u16 = 11;"));
    assert!(DNS.contains("pub const E_SERVFAIL: u16 = 10;"));
}

#[test]
fn an_echo_request_carries_a_valid_checksum_and_its_number() {
    for seq in [1u16, 4, 0xFFFF] {
        let m = echo_request(seq);
        assert_eq!(m.len(), ECHO_LEN);
        assert_eq!(m[0], 8, "echo request");
        assert_eq!(checksum(&m), 0, "a message with its checksum sums to zero");
        assert_eq!(u16::from_be_bytes([m[4], m[5]]), ECHO_ID);
        assert_eq!(u16::from_be_bytes([m[6], m[7]]), seq);
    }
    assert_eq!(checksum(&[0x45, 0x00, 0x00, 0x1c]), !0x451c, "odd and even words");
    assert_eq!(checksum(&[0xFF]), !0xFF00, "a trailing byte is the high half");
}

fn reply(src: [u8; 4], proto: u8, ty: u8, id: u16, seq: u16) -> Vec<u8> {
    let mut b = src.to_vec();
    b.extend_from_slice(&[10, 0, 2, 15, proto, ty, 0, 0, 0]);
    b.extend_from_slice(&id.to_be_bytes());
    b.extend_from_slice(&seq.to_be_bytes());
    b
}

#[test]
fn only_the_reply_to_this_echo_counts() {
    let dst = [10, 0, 2, 2];
    assert!(is_reply(&reply(dst, PROTO_ICMP, 0, ECHO_ID, 3), dst, 3));
    assert!(!is_reply(&reply([10, 0, 2, 3], PROTO_ICMP, 0, ECHO_ID, 3), dst, 3), "another host");
    assert!(!is_reply(&reply(dst, 6, 0, ECHO_ID, 3), dst, 3), "not ICMP");
    assert!(!is_reply(&reply(dst, PROTO_ICMP, 8, ECHO_ID, 3), dst, 3), "a request");
    assert!(!is_reply(&reply(dst, PROTO_ICMP, 0, 0x4E4F, 3), dst, 3), "the terminal's echo");
    assert!(!is_reply(&reply(dst, PROTO_ICMP, 0, ECHO_ID, 2), dst, 3), "an earlier echo");
    let whole = reply(dst, PROTO_ICMP, 0, ECHO_ID, 3);
    for cut in 0..whole.len() {
        assert!(!is_reply(&whole[..cut], dst, 3), "cut at {cut}");
    }
}

#[test]
fn every_send_errno_comes_to_an_answer() {
    assert_eq!(sent(Some(0)), Sent::Yes);
    assert_eq!(sent(Some(7)), Sent::Again, "ARP in flight");
    assert_eq!(sent(Some(6)), Sent::Stop(Echo::NoRoute));
    assert_eq!(sent(Some(5)), Sent::Stop(Echo::NotReady));
    assert_eq!(sent(Some(13)), Sent::Stop(Echo::Failed));
    assert_eq!(sent(None), Sent::Stop(Echo::Unavailable));
    for (name, v) in [("E_NO_CONFIG", 5), ("E_NO_ROUTE", 6), ("E_NO_NEIGHBOUR", 7)] {
        assert!(IP.contains(&format!("pub const {name}: u16 = {v};")), "{name}");
    }
    assert!(IP.contains("pub const MAX_PAYLOAD: usize = 512;"));
}

#[test]
fn a_header_names_its_body_length() {
    let h = header(0x4E49_5034, 4, 21).expect("fits");
    assert_eq!(u32::from_le_bytes([h[16], h[17], h[18], h[19]]), 21);
    assert_eq!(u16::from_le_bytes([h[6], h[7]]), 4);
}

#[test]
fn neither_is_asked_for_off_the_direct_route() {
    for hook in ["fn resolve(", "fn echo("] {
        let at = HOST.find(hook).unwrap_or_else(|| panic!("{hook} is gone"));
        let body = &HOST[at..];
        let gate = body.find("nonos_route_link::direct_refusal()").expect("the route is read");
        let ask = body.find("net_ask::").expect("the call");
        assert!(gate < ask, "{hook} reads the route before it asks");
    }
    assert!(ASK.contains("const ECHO_WAIT_MS: i64 = 1_000;"), "each echo is bounded");
    assert!(ASK.contains("const DNS_WAIT_MS: u64 = 6_000;"), "a lookup is bounded");
}
