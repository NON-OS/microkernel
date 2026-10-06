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

//! The DHCP client reads its replies off the link itself, with no net.ip
//! below it, so its own framing has to check what net.ip would have.

use crate::dhcp::{build_request, parse, Message, CLIENT_PORT, SERVER_PORT};
use crate::frame::dhcp_payload;

extern crate alloc;
use alloc::vec::Vec;

const SERVER: [u8; 4] = [192, 168, 1, 1];
const MAC: [u8; 6] = [2, 0, 0, 0, 0, 9];

fn fold(bytes: &[u8]) -> u16 {
    let mut sum: u32 = 0;
    for pair in bytes.chunks(2) {
        sum += u32::from(u16::from_be_bytes([pair[0], *pair.get(1).unwrap_or(&0)]));
    }
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

/// A server's reply frame around `bootp`, every checksum sealed; `udp_ck`
/// false leaves the UDP checksum zero (none).
fn reply_frame(bootp: &[u8], udp_ck: bool) -> Vec<u8> {
    let dst = [255, 255, 255, 255];
    let udp_len = 8 + bootp.len();
    let mut udp = Vec::new();
    udp.extend_from_slice(&SERVER_PORT.to_be_bytes());
    udp.extend_from_slice(&CLIENT_PORT.to_be_bytes());
    udp.extend_from_slice(&(udp_len as u16).to_be_bytes());
    udp.extend_from_slice(&[0, 0]);
    udp.extend_from_slice(bootp);
    if udp_ck {
        let mut pseudo = Vec::new();
        pseudo.extend_from_slice(&SERVER);
        pseudo.extend_from_slice(&dst);
        pseudo.extend_from_slice(&[0, 17]);
        pseudo.extend_from_slice(&(udp_len as u16).to_be_bytes());
        pseudo.extend_from_slice(&udp);
        let ck = match fold(&pseudo) {
            0 => 0xFFFF,
            c => c,
        };
        udp[6..8].copy_from_slice(&ck.to_be_bytes());
    }
    let total = (20 + udp.len()) as u16;
    let mut ip = alloc::vec![0x45, 0, (total >> 8) as u8, total as u8, 0, 7, 0, 0, 64, 17, 0, 0];
    ip.extend_from_slice(&SERVER);
    ip.extend_from_slice(&dst);
    let ck = fold(&ip);
    ip[10..12].copy_from_slice(&ck.to_be_bytes());
    let mut f = alloc::vec![0xFF; 6];
    f.extend_from_slice(&[2, 0, 0, 0, 0, 1]);
    f.extend_from_slice(&[0x08, 0x00]);
    f.extend_from_slice(&ip);
    f.extend_from_slice(&udp);
    f
}

/// An OFFER for `MAC`, built by the capsule's own request builder and turned
/// into a reply.
fn offer() -> Vec<u8> {
    let msg = Message::new_request(&MAC, 0x1234_5678);
    let mut out = [0u8; 400];
    let n = build_request(&msg, 2, Some([192, 168, 1, 50]), Some(SERVER), &mut out).expect("built");
    let mut b = out[..n].to_vec();
    b[0] = 2;
    b
}

#[test]
fn a_well_formed_reply_is_read() {
    let f = reply_frame(&offer(), true);
    assert_eq!(dhcp_payload(&f), Some(offer().as_slice()));
    assert_eq!(dhcp_payload(&reply_frame(&offer(), false)), Some(offer().as_slice()), "no checksum");
}

/*
 * net.ip checks the IPv4 header checksum, refuses fragments and bounds the
 * datagram by its total length; the DHCP client, framing for itself, did
 * none of that, so a corrupted or fragmented frame was read as a lease.
 */
#[test]
fn a_damaged_ip_header_is_not_a_reply() {
    let mut f = reply_frame(&offer(), true);
    f[14 + 8] ^= 0x01; // the TTL, so only the header checksum can tell
    assert_eq!(dhcp_payload(&f), None);
}

#[test]
fn a_fragment_is_not_a_reply() {
    for flags in [[0x20, 0x00], [0x00, 0x10]] {
        let mut f = reply_frame(&offer(), true);
        f[14 + 6..14 + 8].copy_from_slice(&flags);
        f[14 + 10..14 + 12].copy_from_slice(&[0, 0]);
        let ck = fold(&f[14..34]);
        f[14 + 10..14 + 12].copy_from_slice(&ck.to_be_bytes());
        assert_eq!(dhcp_payload(&f), None, "flags {flags:?}");
    }
}

/// RFC 1122 4.1.3.4: a UDP checksum that is present is verified.
#[test]
fn a_reply_whose_udp_checksum_fails_is_refused() {
    let mut f = reply_frame(&offer(), true);
    let last = f.len() - 1;
    f[last] ^= 0x01;
    assert_eq!(dhcp_payload(&f), None);
}

/// Ethernet pads short frames; what follows the IP total length is padding,
/// not datagram, and a UDP length that runs into it is refused.
#[test]
fn the_ip_total_length_bounds_the_datagram() {
    let mut f = reply_frame(&offer(), false);
    f.extend_from_slice(&[0u8; 32]);
    assert_eq!(dhcp_payload(&f), Some(offer().as_slice()), "padding after the packet is ignored");
    let udp_len_at = 14 + 20 + 4;
    let long = (8 + offer().len() + 16) as u16;
    f[udp_len_at..udp_len_at + 2].copy_from_slice(&long.to_be_bytes());
    assert_eq!(dhcp_payload(&f), None, "a UDP length past the IP packet");
}

#[test]
fn what_the_builder_writes_parses_back() {
    let Ok(m) = parse(&offer()) else { panic!("parses") };
    assert_eq!((m.op, m.xid, m.message_type), (2, 0x1234_5678, 2));
    assert_eq!(&m.chaddr[..6], &MAC);
    assert_eq!(m.server_id, SERVER);
}

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

#[test]
fn random_and_damaged_frames_never_panic_and_stay_in_bounds() {
    let good = reply_frame(&offer(), true);
    let mut s = 0xDC0F_0001u32;
    for round in 0..100_000u32 {
        let f: Vec<u8> = if round % 2 == 0 {
            let mut f = good.clone();
            for _ in 0..1 + xorshift(&mut s) % 3 {
                let at = xorshift(&mut s) as usize % f.len();
                f[at] = xorshift(&mut s) as u8;
            }
            f.truncate(xorshift(&mut s) as usize % (f.len() + 1));
            f
        } else {
            (0..xorshift(&mut s) % 400).map(|_| xorshift(&mut s) as u8).collect()
        };
        if let Some(p) = dhcp_payload(&f) {
            let (f0, p0) = (f.as_ptr() as usize, p.as_ptr() as usize);
            assert!(p0 >= f0 + 42 && p0 + p.len() <= f0 + f.len());
            let _ = parse(p);
        }
    }
}
