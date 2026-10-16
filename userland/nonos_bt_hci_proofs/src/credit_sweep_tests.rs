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
//! Number of Completed Packets lied about (Vol 4, Part E, 7.7.19): a count
//! gives back no more than the link it names holds; an event whose count
//! lies is refused and gives back nothing.

use crate::sim::config::Config;
use crate::sim::stack::stack;

const SEEDS: [u64; 4] = [0x9e37_79b9_7f4a_7c15, 0x2545_f491_4f6c_dd1d, 7, 0xdead_beef];

fn draw(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

// Up to three (handle, count) pairs, half naming the link that holds buffers.
fn pairs(r: &mut u64) -> Vec<(u16, u16)> {
    let link = |r: &mut u64| [0x0040, draw(r) as u16][(draw(r) % 2) as usize];
    (0..draw(r) % 4).map(|_| (link(r), draw(r) as u16 % 6)).collect()
}

#[test]
fn no_completed_count_gives_more_buffers_than_the_controller_has() {
    for mut r in SEEDS {
        let mut s = stack(Config::default());
        s.bring_up().unwrap();
        let (total, mut held) = (s.acl_free(), 0u16);
        for _ in 0..500 {
            for _ in 0..draw(&mut r) % (u64::from(s.acl_free()) + 1) {
                s.send_pdu(0x0040, &[0; 4]).unwrap();
                held += 1;
            }
            let before = s.acl_free();
            let pairs = pairs(&mut r);
            let mut ev = vec![0x13, (1 + 4 * pairs.len()) as u8, pairs.len() as u8];
            pairs.iter().for_each(|(h, n)| ev.extend([h.to_le_bytes(), n.to_le_bytes()].concat()));
            let mut back = 0;
            for &(_, n) in pairs.iter().filter(|p| p.0 & 0x0fff == 0x0040) {
                back += n.min(held - back);
            }
            let honest = !draw(&mut r).is_multiple_of(3);
            // The count lies, one way or the other; the event's own length
            // octet is the transport's framing, so it stays true.
            if !honest {
                let up = draw(&mut r).is_multiple_of(2) || pairs.is_empty();
                ev[2] = if up { ev[2] + 1 + (draw(&mut r) % 3) as u8 } else { ev[2] - 1 };
            }
            s.transport().pipes().send_event(&ev);
            let got = s.next_event();
            if honest {
                assert_eq!(got, Ok(None), "taken by the host");
                assert!(s.acl_free() == before + back && s.acl_free() <= total);
                held -= back;
            } else {
                assert!(got.is_err() && s.acl_free() == before, "a lying count gives nothing");
            }
        }
    }
}
