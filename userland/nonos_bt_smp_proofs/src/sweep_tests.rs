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
//! SMP lies (Vol 3, Part H, 3.5.1 and 3.5.2): Pairing parameters read as
//! sent when sound, and a lying length, key size, OOB flag or IO
//! capability is refused.

use nonos_bt_smp::pdu::Pairing;

pub const SEEDS: [u64; 4] = [0x9e37_79b9_7f4a_7c15, 0x2545_f491_4f6c_dd1d, 7, 0xdead_beef];

pub fn draw(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[test]
fn pairing_parameters_read_as_sent_and_lies_are_refused() {
    for mut s in SEEDS {
        for _ in 0..3000 {
            let (r, t) = (draw(&mut s).to_le_bytes(), draw(&mut s).to_le_bytes());
            let p = [r[0] % 5, r[1] % 2, r[2], 7 + r[3] % 10, r[4], r[5]];
            assert_eq!(Pairing::parse(&p).map(|x| x.params()), Ok(p));
            let n = usize::from(r[6] % 12);
            let cut = [&p[..], &r[..]].concat();
            assert_eq!(Pairing::parse(&cut[..n]).is_err(), n != 6, "{n} octets");
            let lie = |i: usize, v: u8| {
                let mut q = p;
                q[i] = v;
                Pairing::parse(&q).is_err()
            };
            assert!(lie(3, if r[7] < 128 { r[7] % 7 } else { 17 + r[7] % 239 }), "a key size");
            assert!(lie(1, 2 + t[0] % 254), "an OOB flag");
            assert!(lie(0, 5 + t[1] % 251), "an IO capability");
        }
    }
}
