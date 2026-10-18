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
//! L2CAP basic frames and LE signaling commands (Vol 3, Part A, 3.1 and
//! 4): sound ones read as built; a length that lies either way, or
//! identifier 0, is refused.

use super::rng::each;
use nonos_bt_wire::l2cap::{basic, parse_basic, parse_signal, signal};

#[test]
fn frames_and_commands_read_as_built_and_lying_lengths_are_refused() {
    each(|rng| {
        let (cid, payload) = (rng.next() as u16, rng.span(0, 100));
        let f = basic(cid, &payload).unwrap();
        assert_eq!(parse_basic(&f), Ok((cid, &payload[..])));
        let (code, id) = (rng.byte(), 1 + rng.below(255) as u8);
        let s = signal(code, id, &payload).unwrap();
        let got = parse_signal(&s).expect("sound");
        assert_eq!((got.code, got.id, got.data), (code, id, &payload[..]));
        for (mut lie, parse) in [(f, 0), (s, 2)] {
            let n = (payload.len() as u16).wrapping_add(1 + rng.below(0xfffe) as u16);
            lie[parse..parse + 2].copy_from_slice(&n.to_le_bytes());
            let refused =
                if parse == 0 { parse_basic(&lie).is_err() } else { parse_signal(&lie).is_err() };
            assert!(refused, "a stated length of {n}");
        }
        let mut zero = signal(code, id, &payload).unwrap();
        zero[1] = 0;
        assert!(parse_signal(&zero).is_err(), "identifier 0");
    });
}
