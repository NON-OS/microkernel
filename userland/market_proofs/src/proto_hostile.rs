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

//! Hostile answers from the market (the owner's rule 8): random bodies of
//! every short length and longer, zero and 0xff fills, through every parser
//! the store, the wallet and the meter read the market with. None may panic,
//! and every reason code has a name.

use nonos_market_proto::*;

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

#[test]
fn no_parser_panics_on_hostile_bodies() {
    let lens = (0..400).chain([1023, 1024, 4096, 65_536]);
    for b in lens.flat_map(|n| [bytes(n as u64 + 5, n), vec![0; n], vec![0xff; n]]) {
        let _ = (parse_app(&b), parse_commerce(&b), parse_list(&b), parse_readiness(&b));
        let _ = (parse_release(&b), parse_release_escrow(&b), reply_body(&b, 1, 1));
        let _ = installed_line(&b);
    }
}

#[test]
fn every_code_and_status_is_named_or_none() {
    for code in 0..=u8::MAX {
        let _ = (reason(code), removal_only(code), BondCode::from_byte(code));
    }
    for status in [i32::MIN, -1, 0, 1, i32::MAX] {
        let _ = status_name(status);
    }
}
