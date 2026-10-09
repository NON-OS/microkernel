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

//! What an inbound ARP packet may teach the neighbour cache, and when it is
//! answered, through the real L2 handler.

use crate::arp::packet::{ArpPacket, OPER_REPLY, OPER_REQUEST, PACKET_LEN};
use crate::arp::{on_inbound, Cache, Iface};

const US: Iface = Iface { mac: [2, 0, 0, 0, 0, 1], ipv4: [10, 0, 2, 15] };
const PEER_MAC: [u8; 6] = [2, 0, 0, 0, 0, 2];
const PEER_IP: [u8; 4] = [10, 0, 2, 2];

fn packet(oper: u16, sender_mac: [u8; 6], sender_ip: [u8; 4], target_ip: [u8; 4]) -> [u8; PACKET_LEN] {
    let mut out = [0u8; PACKET_LEN];
    let p = ArpPacket { oper, sender_mac, sender_ip, target_mac: [0; 6], target_ip };
    assert!(p.write(&mut out));
    out
}

#[test]
fn a_request_for_us_is_answered_and_its_sender_learned() {
    let mut c = Cache::new();
    let reply = on_inbound(&US, &mut c, &packet(OPER_REQUEST, PEER_MAC, PEER_IP, US.ipv4));
    let reply = reply.expect("answered");
    assert_eq!(&reply.bytes[0..6], &PEER_MAC, "to the asker's MAC");
    assert_eq!(c.lookup(&PEER_IP), Some(PEER_MAC));
}

/*
 * An address probe (RFC 5227) has sender 0.0.0.0. It is answered, which is
 * how a host defends its address, but 0.0.0.0 is nobody's address and was
 * learned as a neighbour all the same.
 */
#[test]
fn a_probe_is_answered_but_0_0_0_0_is_not_learned() {
    let mut c = Cache::new();
    assert!(on_inbound(&US, &mut c, &packet(OPER_REQUEST, PEER_MAC, [0; 4], US.ipv4)).is_some());
    assert_eq!(c.lookup(&[0; 4]), None);
}

/*
 * A sender whose hardware address is a group address (its first bit set),
 * broadcast included, or zero is no neighbour: learning it pointed a unicast
 * IP at every host on the segment, and the reply went to all of them.
 */
#[test]
fn a_sender_with_a_group_or_zero_mac_is_neither_learned_nor_answered() {
    for mac in [[0xFF; 6], [0x01, 0x00, 0x5E, 0, 0, 1], [0x33, 0x33, 0, 0, 0, 1], [0; 6]] {
        let mut c = Cache::new();
        assert!(on_inbound(&US, &mut c, &packet(OPER_REQUEST, mac, PEER_IP, US.ipv4)).is_none());
        assert_eq!(c.lookup(&PEER_IP), None, "{mac:?}");
    }
}

/// Our own address, a broadcast or multicast address is not a neighbour's.
#[test]
fn a_sender_address_no_neighbour_can_have_is_not_learned() {
    for ip in [US.ipv4, [255, 255, 255, 255], [224, 0, 0, 1], [127, 0, 0, 1]] {
        let mut c = Cache::new();
        let _ = on_inbound(&US, &mut c, &packet(OPER_REQUEST, PEER_MAC, ip, US.ipv4));
        assert_eq!(c.lookup(&ip), None, "{ip:?}");
    }
}

/// Before DHCP has given us an address there is none to defend or answer for.
#[test]
fn an_unconfigured_host_answers_nothing() {
    let bare = Iface { mac: US.mac, ipv4: [0; 4] };
    let mut c = Cache::new();
    assert!(on_inbound(&bare, &mut c, &packet(OPER_REQUEST, PEER_MAC, PEER_IP, [0; 4])).is_none());
}

/// RFC 826 has two operations; a packet with any other is not ARP to learn from.
#[test]
fn only_requests_and_replies_teach_the_cache() {
    for oper in [0u16, 3, 4, 0xFFFF] {
        let mut c = Cache::new();
        c.note_pending(PEER_IP);
        let _ = on_inbound(&US, &mut c, &packet(oper, PEER_MAC, PEER_IP, US.ipv4));
        assert_eq!(c.lookup(&PEER_IP), None, "oper {oper}");
    }
    let mut c = Cache::new();
    c.note_pending(PEER_IP);
    let _ = on_inbound(&US, &mut c, &packet(OPER_REPLY, PEER_MAC, PEER_IP, US.ipv4));
    assert_eq!(c.lookup(&PEER_IP), Some(PEER_MAC), "a reply to our request is learned");
}

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

/// Over random packets, mostly well formed, the cache only ever holds
/// unicast neighbours, and only requests for our address are answered.
#[test]
fn random_packets_leave_only_real_neighbours_in_the_cache() {
    let mut s = 0xA4B0_0001u32;
    let mut c = Cache::new();
    let mut seen: alloc::vec::Vec<[u8; 4]> = alloc::vec::Vec::new();
    for _ in 0..100_000 {
        let mut mac = [0u8; 6];
        mac.iter_mut().for_each(|b| *b = xorshift(&mut s) as u8);
        let ip = match xorshift(&mut s) % 4 {
            0 => (xorshift(&mut s)).to_be_bytes(),
            1 => US.ipv4,
            _ => [10, 0, 2, (xorshift(&mut s) % 8) as u8],
        };
        let target = if xorshift(&mut s).is_multiple_of(2) { US.ipv4 } else { xorshift(&mut s).to_be_bytes() };
        let oper = (xorshift(&mut s) % 4) as u16;
        if xorshift(&mut s).is_multiple_of(3) {
            c.note_pending(ip);
        }
        let mut p = packet(oper, mac, ip, target);
        if xorshift(&mut s).is_multiple_of(8) {
            let at = xorshift(&mut s) as usize % PACKET_LEN;
            p[at] = xorshift(&mut s) as u8;
        }
        let answered = on_inbound(&US, &mut c, &p).is_some();
        if answered {
            let parsed = ArpPacket::parse(&p).expect("only a whole packet is answered");
            assert!(parsed.oper == OPER_REQUEST && parsed.target_ip == US.ipv4);
        }
        seen.push(ip);
    }
    for ip in seen {
        if let Some(mac) = c.lookup(&ip) {
            assert!(ip[0] != 0 && ip[0] != 127 && ip[0] < 224 && ip != US.ipv4, "{ip:?}");
            assert!(mac[0] & 1 == 0 && mac != [0; 6], "{mac:?}");
        }
    }
}
