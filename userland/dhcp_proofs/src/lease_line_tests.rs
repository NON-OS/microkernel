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

//! The "[DHCP] lease" line byte for byte, `none` for an absent address, and
//! a sweep of addresses parsed back to the ones given.

use crate::lease_line::{lease_line, LINE_MAX};

fn line(ip: [u8; 4], prefix: u8, gw: [u8; 4], server: [u8; 4]) -> String {
    let (buf, len) = lease_line(&ip, prefix, &gw, &server);
    String::from_utf8(buf[..len].to_vec()).unwrap()
}

#[test]
fn qemus_user_net_lease_reads_as_the_lead_expects() {
    let got = line([10, 0, 2, 15], 24, [10, 0, 2, 2], [10, 0, 2, 2]);
    assert_eq!(got, "[DHCP] lease 10.0.2.15/24 gw 10.0.2.2 from 10.0.2.2\n");
}

#[test]
fn an_address_the_server_left_out_is_none_not_zero() {
    assert_eq!(
        line([192, 168, 1, 7], 16, [0; 4], [0; 4]),
        "[DHCP] lease 192.168.1.7/16 gw none from none\n"
    );
}

#[test]
fn every_octet_and_prefix_has_no_leading_zero_and_reads_back() {
    let mut seed = 0x4e4f_4e4fu32;
    for round in 0..4096u32 {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        let b = seed.to_le_bytes();
        let ip = [(round & 0xff) as u8, b[1], b[2], b[3] | 1];
        let gw = [b[3], b[0], (round >> 4) as u8, b[1] | 1];
        let prefix = (round % 32 + 1) as u8;
        let got = line(ip, prefix, gw, ip);
        let rest = got.strip_prefix("[DHCP] lease ").unwrap().strip_suffix('\n').unwrap();
        let words: Vec<&str> = rest.split(' ').collect();
        assert_eq!(words.len(), 5, "{got}");
        let (addr, bits) = words[0].split_once('/').unwrap();
        assert_eq!(bits.parse::<u8>().unwrap(), prefix);
        for (text, want) in [(addr, ip), (words[2], gw), (words[4], ip)] {
            let parsed: Vec<u8> = text
                .split('.')
                .map(|o| {
                    assert!(o == "0" || !o.starts_with('0'), "{got}");
                    o.parse().unwrap()
                })
                .collect();
            assert_eq!(parsed, want, "{got}");
        }
    }
}

#[test]
fn the_longest_line_fits_whole_with_its_newline() {
    let max = [255, 255, 255, 255];
    let got = line(max, 32, max, max);
    assert_eq!(got, "[DHCP] lease 255.255.255.255/32 gw 255.255.255.255 from 255.255.255.255\n");
    assert!(got.len() <= LINE_MAX);
}
