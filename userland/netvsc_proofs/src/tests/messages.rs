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

//! The messages byte by byte against OpenVMM's netvsp_protocol and Linux's
//! rndis_filter.c, and the DISCOVER against the egress rules.

use crate::nvsp;
use crate::probe::{discover, Probe, Step};
use crate::rndis::{self, Received};

fn w(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes(b[at..at + 4].try_into().unwrap())
}

#[test]
fn nvsp_messages_are_their_type_and_fields_padded_to_forty_bytes() {
    assert_eq!(nvsp::init(0x60001)[..12], [1, 0, 0, 0, 1, 0, 6, 0, 1, 0, 6, 0]);
    let m = nvsp::buffer(101, 0x4E45_1011, 0xCAFE);
    assert_eq!((w(&m, 0), w(&m, 4), w(&m, 8), m.len()), (101, 0x4E45_1011, 0xCAFE, 40));
    let m = nvsp::ndis_config();
    assert_eq!((w(&m, 0), w(&m, 4), w(&m, 8)), (125, 1514, 0));
    assert_eq!(nvsp::rndis_done()[..8], [108, 0, 0, 0, 1, 0, 0, 0]);
}

#[test]
fn completions_are_read_only_when_they_say_success() {
    let mut a = [0u8; 40];
    a[0] = 2;
    a[12] = 1;
    assert!(nvsp::init_taken(&a));
    a[12] = 0;
    assert!(!nvsp::init_taken(&a), "status NONE: refused");
    let mut r = [0u8; 40];
    for (i, v) in [102u32, 1, 1, 0, 1806, 145, 1806 * 145].iter().enumerate() {
        r[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
    assert_eq!(nvsp::receive_buffer(&r), Some((1806, 145)));
    for (at, bad) in [(4, 2u32), (8, 0), (12, 64), (16, 0), (20, 0)] {
        let mut b = r;
        b[at..at + 4].copy_from_slice(&bad.to_le_bytes());
        assert_eq!(nvsp::receive_buffer(&b), None, "field at {at}");
    }
    assert_eq!(nvsp::send_buffer(&[105, 0, 0, 0, 1, 0, 0, 0, 0, 24, 0, 0]), Some(6144));
    assert_eq!(nvsp::send_buffer(&[105, 0, 0, 0, 2, 0, 0, 0, 0, 24, 0, 0]), None);
    assert_eq!(nvsp::send_buffer(&[105, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]), None);
}

#[test]
fn the_network_address_parameter_is_linuxs() {
    let p = rndis::network_address(&[0x02, 0xAB, 0, 1, 0xFE, 0x10]);
    assert_eq!((w(&p, 0), w(&p, 4), w(&p, 8), w(&p, 12), w(&p, 16)), (20, 28, 2, 48, 24));
    let name: Vec<u16> = p[20..48].chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    let value: Vec<u16> = p[48..72].chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
    assert_eq!(String::from_utf16(&name).unwrap(), "NetworkAddress");
    assert_eq!(String::from_utf16(&value).unwrap(), "02ab0001fe10", "%pm: lowercase, no separators");
}

#[test]
fn rndis_requests_lay_out_as_rndisprot() {
    let q = rndis::query(9, 0x0101_0101);
    assert_eq!((w(&q, 0), w(&q, 4), w(&q, 8), w(&q, 12), w(&q, 16), w(&q, 20)), (4, 28, 9, 0x0101_0101, 0, 20));
    let s = rndis::set(3, 0x0001_010E, &13u32.to_le_bytes());
    assert_eq!((w(&s, 4), w(&s, 16), w(&s, 20), w(&s, 28)), (32, 4, 20, 13));
    let h = rndis::packet_header(60);
    assert_eq!((w(&h, 0), w(&h, 4), w(&h, 8), w(&h, 12)), (1, 104, 36, 60));
    assert_eq!(&h[16..], &[0u8; 28], "no out-of-band data, no per-packet info, no VC");
}

#[test]
fn messages_from_the_host_are_held_to_their_own_lengths() {
    let mut q = vec![0u8; 30];
    for (i, v) in [0x8000_0004u32, 30, 7, 0, 6, 16].iter().enumerate() {
        q[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
    assert_eq!(rndis::read(&q), Some(Received::Reply { request: 7, status: 0, info: (24, 6) }));
    q[16] = 7;
    assert_eq!(rndis::read(&q), None, "information past the message");
    let mut p = vec![0u8; 300];
    for (i, v) in [1u32, 300, 248, 44].iter().enumerate() {
        p[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
    assert_eq!(rndis::read(&p), Some(Received::Frame(256, 44)));
    p[12] = 45;
    assert_eq!(rndis::read(&p), None, "a frame past the message");
    p[12] = 0;
    assert_eq!(rndis::read(&p), None, "an empty frame");
    let mut st = vec![0u8; 20];
    for (i, v) in [7u32, 20, 0x4001_000C].iter().enumerate() {
        st[4 * i..4 * i + 4].copy_from_slice(&v.to_le_bytes());
    }
    assert_eq!(rndis::read(&st), Some(Received::Link(false)));
    assert_eq!(rndis::read(&st[..7]), None);
}

#[test]
fn the_discover_carries_only_what_the_egress_rules_allow() {
    let mac = [0x02, 1, 2, 3, 4, 5];
    let f = discover(&mac, 0xDEAD_BEEF);
    assert_eq!(&f[0..6], &[0xFF; 6]);
    assert_eq!(&f[6..12], &mac);
    assert_eq!(&f[30..34], &[255; 4], "to the broadcast address");
    assert_eq!(&f[26..30], &[0; 4], "from 0.0.0.0");
    let bootp = &f[42..];
    assert_eq!(&bootp[4..8], &0xDEAD_BEEFu32.to_be_bytes());
    assert_eq!(&bootp[28..34], &mac);
    assert!(bootp[44..236].iter().all(|b| *b == 0), "sname and file empty");
    assert_eq!(&bootp[240..], &[53, 1, 1, 255], "option 53 alone");
    // The IPv4 header's checksum holds.
    let sum: u32 = f[14..34].chunks(2).map(|c| u32::from(u16::from_be_bytes([c[0], c[1]]))).sum();
    assert_eq!((sum & 0xFFFF) + (sum >> 16), 0xFFFF);
}

#[test]
fn the_probe_sends_its_tries_then_waits_once_more_then_gives_up() {
    let mut p = Probe::new();
    let mut steps = Vec::new();
    let mut t = 0;
    while t <= 8000 {
        let s = p.step(t, 3, 2000);
        if s == Step::Send {
            p.sent_at(t);
        }
        if s != Step::Listen {
            steps.push((t, s));
        }
        t += 500;
    }
    assert_eq!(steps[..4], [(0, Step::Send), (2000, Step::Send), (4000, Step::Send), (6000, Step::GiveUp)]);
    assert_eq!(Probe::new().step(5, 0, 2000), Step::GiveUp, "no tries: given up at once");
}

#[test]
fn only_an_rndis_packet_message_names_its_channel() {
    assert_eq!(nvsp::received(&nvsp::rndis_packet(1, u32::MAX, 0)), Some(1));
    assert_eq!(nvsp::received(&nvsp::rndis_packet(0, u32::MAX, 0)), Some(0));
    assert_eq!(nvsp::received(&nvsp::rndis_done()), None);
    assert_eq!(nvsp::received(&[107, 0, 0]), None, "too short");
}

#[test]
fn the_discovers_lengths_are_its_own() {
    let f = discover(&[2, 0, 0, 0, 0, 1], 1);
    assert_eq!(f.len(), 14 + 20 + 8 + 244);
    assert_eq!(u16::from_be_bytes([f[16], f[17]]), 20 + 8 + 244, "IPv4 total length");
    assert_eq!(u16::from_be_bytes([f[38], f[39]]), 8 + 244, "UDP length");
}

#[test]
fn a_frame_too_short_or_of_another_kind_is_never_an_answer() {
    use crate::probe::answers;
    let mac = [2, 1, 1, 1, 1, 1];
    let mut wire = crate::model::wire::Wire::new();
    let offer = wire.answer(&discover(&mac, 7)).remove(0);
    assert!(answers(&offer, &mac, 7));
    for cut in [0, 13, 33, 34, 41, 42 + 239] {
        assert!(!answers(&offer[..cut], &mac, 7), "cut at {cut}");
    }
    let mut tcp = offer.clone();
    tcp[23] = 6;
    assert!(!answers(&tcp, &mac, 7), "not UDP");
    let mut v6 = offer.clone();
    v6[12..14].copy_from_slice(&[0x86, 0xDD]);
    assert!(!answers(&v6, &mac, 7), "not IPv4");
    let mut request = offer.clone();
    request[42] = 1;
    assert!(!answers(&request, &mac, 7), "a BOOTREQUEST");
    let mut ports = offer.clone();
    ports[34..38].copy_from_slice(&[0, 68, 0, 67]);
    assert!(!answers(&ports, &mac, 7), "client to server");
    // IPv4 options move the UDP header; the reply is still read.
    let mut opts = offer[..14].to_vec();
    opts.extend_from_slice(&offer[14..34]);
    opts[14] = 0x46;
    opts.extend_from_slice(&[1, 1, 1, 0]);
    opts.extend_from_slice(&offer[34..]);
    assert!(answers(&opts, &mac, 7));
}

#[test]
fn a_dhcp_reply_with_no_options_is_still_an_answer() {
    use crate::probe::answers;
    let mac = [2, 3, 3, 3, 3, 3];
    let mut wire = crate::model::wire::Wire::new();
    let offer = wire.answer(&discover(&mac, 9)).remove(0);
    // The UDP payload cut to the 240 fixed bytes: 248 with its header.
    let short = &offer[..14 + 20 + 8 + 240];
    assert!(answers(short, &mac, 9));
    assert!(!answers(&short[..short.len() - 1], &mac, 9));
}
