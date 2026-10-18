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

//! The framer keeps nothing but the prefix it is in the middle of: after a
//! key, whatever came before it, it is idle again, and an answer to a
//! command leaves a key sequence where it was.

use crate::prefix::{Prefix, Scan};

#[test]
fn every_byte_after_every_prefix_leaves_the_framer_idle_again() {
    for prefix in [&[][..], &[0xE0], &[0xE1, 0x1D]] {
        for b in 0u8..=0xFF {
            let mut p = Prefix::IDLE;
            prefix.iter().for_each(|&x| _ = p.take(x));
            let before = p;
            let scan = p.take(b);
            if matches!(b, 0xFA | 0xFE) {
                // An answer to a command, wherever it lands, is not part of
                // the key sequence and leaves it where it was.
                assert_eq!((scan, p), (Scan::Reply(b), before), "{prefix:02X?} {b:02X}");
            } else if let Scan::Key { .. } = scan {
                assert_eq!(p, Prefix::IDLE, "{prefix:02X?} {b:02X}");
            }
        }
    }
    let mut p = Prefix::IDLE;
    for b in [0xE1, 0xE1, 0xE1, 0xE0, 0xE0, 0x1C] {
        let _ = p.take(b);
    }
    assert_eq!(p, Prefix::IDLE, "a run of prefixes ends where the next key does");
}
