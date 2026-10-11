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

//! Hostile bytes for every decoder of what the meter keeps on disk or hears
//! from the keyring (the owner's rule 8): random bytes of every short length
//! and longer, zero and 0xff fills. None may panic; the persisted state and
//! the sealed file refuse what does not read, and a table that reads
//! encodes back to the same bytes.

mod host;

use host::Host;
use nonos_nox_meter::escrow::KeptOffer;
use nonos_nox_meter::orders::{decode_orders, encode_orders, Order};
use nonos_nox_meter::persist::{decode, mac_key, open};
use nonos_nox_meter::typed::parse_reply;

fn bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x as u8
        })
        .collect()
}

fn hostile() -> impl Iterator<Item = Vec<u8>> {
    let lens = (0..300).chain([511, 512, 1024, 4096, 20_000]);
    lens.flat_map(|n| [bytes(n as u64 + 7, n), vec![0; n], vec![0xff; n]])
}

#[test]
fn no_decoder_panics_on_hostile_bytes() {
    assert_eq!(host::keccak(b"")[..2], [0xc5, 0xd2], "the host hasher is keccak-256");
    let key = mac_key(&Host, &[3; 32]);
    for b in hostile() {
        let _ = (decode(&b), Order::decode(&b), KeptOffer::decode(&b), parse_reply(&b));
        assert!(open(&Host, &key, &b).is_err(), "{} bytes opened", b.len());
        if let Some(t) = decode_orders(&b) {
            assert_eq!(encode_orders(&t), Some(b.clone()), "orders table");
        }
    }
}
