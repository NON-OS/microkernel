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

//! Whether a directed advertisement's target is one of this host's own
//! private addresses (Vol 6, Part B, 1.3.2.3), for every kind of address.

use crate::link::aimed::aimed_here;
use crate::private_tests::IRK;
use nonos_bt_smp::privacy::rpa;
use nonos_bt_wire::addr::BdAddr;

#[test]
fn every_address_made_under_our_key_is_ours() {
    for p in [[0x40, 0, 1], [0x47, 0x12, 0x34], [0x7f, 0xff, 0xfe], [0x55, 0xaa, 0x55]] {
        assert!(aimed_here(&rpa(&IRK, &p), Some(&IRK)), "{p:?}");
        assert!(!aimed_here(&rpa(&IRK, &p), Some(b"another-hosts-ky")), "{p:?}");
        assert!(!aimed_here(&rpa(&IRK, &p), None), "no key, never ours");
    }
}

#[test]
fn no_other_kind_of_address_is_ours_whatever_its_low_octets() {
    let ours = rpa(&IRK, &[0x47, 0x12, 0x34]).0;
    // The same hash and prand under the three other top-bit patterns:
    // non-resolvable (00), reserved (10) and static (11).
    for top in [0x00u8, 0x80, 0xc0] {
        let mut a = ours;
        a[5] = (a[5] & 0x3f) | top;
        assert!(!aimed_here(&BdAddr(a), Some(&IRK)), "{top:#x}");
    }
}

#[test]
fn one_bit_changed_in_the_hash_or_prand_is_not_ours() {
    let ours = rpa(&IRK, &[0x47, 0x12, 0x34]).0;
    for octet in 0..6 {
        for bit in 0..8 {
            if octet == 5 && bit >= 6 {
                continue;
            }
            let mut a = ours;
            a[octet] ^= 1 << bit;
            assert!(!aimed_here(&BdAddr(a), Some(&IRK)), "octet {octet} bit {bit}");
        }
    }
}
