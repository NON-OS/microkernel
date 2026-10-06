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

//! Packets off the wire: the TCP, DHCP, ARP and ICMP parsers never panic, and
//! the payload a parser hands on is a tail of the packet it was given.

#![no_main]

use libfuzzer_sys::fuzz_target;
use net_proofs::arp::packet::ArpPacket;
use net_proofs::{dhcp, icmp, tcp};

fn tail(outer: &[u8], inner: &[u8]) -> bool {
    let (o, i) = (outer.as_ptr() as usize, inner.as_ptr() as usize);
    inner.is_empty() || (i >= o && i + inner.len() == o + outer.len())
}

fuzz_target!(|data: &[u8]| {
    let Some((&which, bytes)) = data.split_first() else {
        return;
    };
    match which % 4 {
        0 => {
            if let Ok((_, payload)) = tcp::parse::parse(&[10, 0, 0, 1], &[10, 0, 0, 2], bytes) {
                assert!(tail(bytes, payload), "the TCP payload is not the segment's tail");
            }
        }
        1 => {
            let _ = dhcp::parse(bytes);
        }
        2 => {
            let _ = ArpPacket::parse(bytes);
        }
        _ => {
            if let Ok((_, payload)) = icmp::parse::parse(bytes) {
                assert!(tail(bytes, payload), "the ICMP payload is not the packet's tail");
            }
        }
    }
});
