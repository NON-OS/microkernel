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

//! Random octets on an H4 line never panic the reader, never yield a
//! packet longer than its kind allows, and every packet yielded is exactly
//! as long as its own header says. A host that leaves events unread keeps
//! the newest `HELD_MAX` and counts the rest.

use super::h4_port::H4Port;
use crate::sim::config::Config;
use nonos_bt_hci::uart::h4::{kind, H4Hci, H4Reader, HELD_MAX};
use nonos_bt_hci::Transport;

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn random_lines_yield_only_packets_their_headers_state() {
    let mut seed = 0x5eed_1234_abcd_0001;
    for _ in 0..20_000 {
        let mut r = H4Reader::new(27);
        let len = (next(&mut seed) % 96) as usize;
        let mut line: Vec<u8> = (0..len).map(|_| next(&mut seed) as u8).collect();
        // Mostly valid indicators, so packets do get through.
        if let Some(b) = line.first_mut() {
            *b = [0x02, 0x03, 0x04, 0x05][(*b % 4) as usize];
        }
        r.push(&line);
        while let Ok(Some((k, p))) = r.take() {
            let (header, max) = kind::shape(k, 27).expect("only kinds a controller sends");
            assert!(p.len() <= max);
            assert_eq!(p.len(), header + kind::stated(k, &p[..header]));
        }
    }
}

#[test]
fn events_left_unread_keep_the_newest_and_count_the_rest() {
    let mut port = H4Port::new(Config::default(), 4096);
    for i in 0..(HELD_MAX as u8 + 4) {
        port.line(&[0x04, 0xff, 0x01, i]);
    }
    let mut h4 = H4Hci::new(port, 27);
    // Looks for ACL data read the line in 64 octets at a time.
    for _ in 0..4 {
        assert_eq!(h4.poll_acl().unwrap(), None);
    }
    let first = h4.poll_event().unwrap().unwrap();
    assert_eq!(first, [0xff, 0x01, 4], "the four oldest were dropped");
    assert_eq!(h4.dropped(), 4);
}
