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

//! H4 framing against a hostile line (Vol 4, Part A, 2 and 3): packets
//! of every kind cut at every octet come out whole and in order; an octet
//! that is not a packet indicator, or a length over its kind's bound, ends
//! the stream for good; nothing is ever read past what was stated.

use nonos_bt_hci::uart::h4::H4Reader;
use nonos_bt_hci::HciError;
use nonos_bt_wire::WireError;

fn stream() -> Vec<u8> {
    let event = [0x04, 0x0e, 0x04, 0x01, 0x03, 0x0c, 0x00];
    let acl = [0x02, 0x40, 0x20, 0x03, 0x00, 0xc0, 0xdb, 0x7e];
    let sco = [0x03, 0x01, 0x00, 0x02, 0x55, 0x66];
    let iso = [0x05, 0x01, 0x00, 0x01, 0xc0, 0x77];
    [&event[..], &acl, &sco, &iso, &[0x04, 0x13, 0x00]].concat()
}

#[test]
fn packets_cut_at_any_octet_come_out_whole_and_in_order() {
    let s = stream();
    for cut in 0..=s.len() {
        let mut r = H4Reader::new(27);
        let mut got = Vec::new();
        for part in [&s[..cut], &s[cut..]] {
            r.push(part);
            while let Some(p) = r.take().unwrap() {
                got.push(p);
            }
        }
        let kinds: Vec<u8> = got.iter().map(|p| p.0).collect();
        assert_eq!(kinds, [0x04, 0x02, 0x03, 0x05, 0x04], "cut at {cut}");
        assert_eq!(got[1].1, [0x40, 0x20, 0x03, 0x00, 0xc0, 0xdb, 0x7e]);
        assert_eq!(
            got[3].1.len(),
            5,
            "ISO length is 14 bits: the top two of 0xc001 are not length"
        );
        assert_eq!(got[4].1, [0x13, 0x00]);
    }
}

#[test]
fn an_octet_that_is_no_packet_indicator_ends_the_stream() {
    for bad in [0x00, 0x01, 0x06, 0x0e, 0xff] {
        let mut r = H4Reader::new(27);
        r.push(&[bad, 0x0e, 0x00]);
        assert_eq!(r.take(), Err(HciError::Wire(WireError::Reserved)), "{bad:#x}");
        r.push(&[0x04, 0x13, 0x00]);
        assert_eq!(
            r.take(),
            Err(HciError::Wire(WireError::Reserved)),
            "no edge to start again from"
        );
    }
}
