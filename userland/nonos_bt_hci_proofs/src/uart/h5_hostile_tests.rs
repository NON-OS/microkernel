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

//! Random octets on an active Three-wire line never panic either side,
//! never put more unacknowledged packets in flight than the window, and
//! never yield a payload longer than the length field can state. And the
//! configuration each side ends with is what both offered (Vol 4, Part D,
//! "Link Establishment").

use super::h5_pair::Pair;
use nonos_bt_hci::uart::h5::header::PAYLOAD_MAX;
use nonos_bt_hci::uart::h5::{Agreed, OURS};

fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn random_lines_into_an_active_link_break_nothing() {
    let mut p = Pair::new(0);
    assert!(p.until(1_000, |p| p.a.active() && p.b.active()));
    let mut seed = 0x0dd_ba11_c0ff_ee00;
    for round in 0..20_000 {
        let len = (next(&mut seed) % 40) as usize;
        let mut line: Vec<u8> = (0..len).map(|_| next(&mut seed) as u8).collect();
        if round % 3 == 0 {
            line.insert(0, 0xc0);
            line.push(0xc0);
        }
        p.a.feed(&line, p.now);
        let _ = p.a.send(1, &[round as u8]);
        p.tick();
        assert!(p.a.unacked() <= usize::from(OURS.window));
        assert!(p.a.inbox.iter().chain(&p.b.inbox).all(|(_, v)| v.len() <= PAYLOAD_MAX));
        p.a.inbox.clear();
        p.b.inbox.clear();
    }
}

#[test]
fn the_link_uses_what_both_sides_offered() {
    let w4_crc = Agreed { window: 4, oof: false, crc: true };
    assert_eq!(OURS, w4_crc);
    assert_eq!(OURS.field(), 0x14);
    assert_eq!(OURS.with(Some(0x17)), w4_crc, "their 7 is more than our 4");
    assert_eq!(OURS.with(Some(0x02)), Agreed { window: 2, oof: false, crc: false });
    assert_eq!(
        OURS.with(Some(0x0f)),
        Agreed { window: 4, oof: false, crc: false },
        "no OOF we did not offer"
    );
    assert_eq!(
        OURS.with(Some(0x10)),
        Agreed { window: 1, oof: false, crc: true },
        "window 0 runs as 1"
    );
    assert_eq!(OURS.with(None), Agreed { window: 1, oof: false, crc: false }, "no field");
}
