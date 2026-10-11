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

//! The device section's reader on hostile bytes. Its happy path runs in
//! verify_tests, over a real proof.

use nonos_proof_abi::{decode_device, DEVICE_HEAD, DEVICE_MAX};

fn sample() -> Vec<u8> {
    let mut b = b"NZKDPRF1".to_vec();
    b.resize(DEVICE_HEAD, 0);
    b[104..108].copy_from_slice(&4u32.to_le_bytes());
    b[188..192].copy_from_slice(&3u32.to_le_bytes());
    b.extend_from_slice(b"abc");
    b
}

#[test]
fn a_well_formed_section_reads() {
    let (st, proof) = decode_device(&sample()).expect("reads");
    assert_eq!((st.device_depth, proof.bytes), (4, b"abc".to_vec()));
}

#[test]
fn hostile_sections_are_refused() {
    let b = sample();
    let mut cases = vec![b[..DEVICE_HEAD].to_vec(), [&b[..], b"x"].concat()];
    for (at, v) in [(0, b'X'), (104, 0), (104, 200), (188, 9)] {
        let mut g = b.clone();
        g[at] = v;
        cases.push(g);
    }
    for at in [8, 108, 116, 124, 156] {
        let mut g = b.clone();
        g[at..at + 8].copy_from_slice(&u64::MAX.to_le_bytes());
        cases.push(g);
    }
    let mut big = b.clone();
    big.resize(DEVICE_MAX + 1, 0);
    cases.push(big);
    for c in cases {
        assert!(decode_device(&c).is_none());
    }
}
