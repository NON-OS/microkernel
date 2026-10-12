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

//! Hostile bytes for every parser and check on settle's wire (the owner's
//! rule 8): random frames of every short length and longer ones, all-zero
//! and all-0xff fills. None may panic, and what parses encodes back to the
//! same bytes.

use nonos_settle_proto::*;

/// A fixed xorshift, so a failure names its seed and repeats.
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
    let lens = (0..300).chain([511, 512, 1024, 4096, 70_000]);
    lens.flat_map(|n| [bytes(n as u64 + 1, n), vec![0; n], vec![0xff; n]])
}

#[test]
fn no_parser_or_check_panics_on_hostile_bytes() {
    for b in hostile() {
        let _ = (Gas::parse(&b), Page::parse(&b), Read::parse(&b), parse_read_reply(&b));
        let _ = (parse_header(&b), ChannelState::parse(&b), Opened::parse(&b));
        let _ = (Settled::parse(&b), Escrow::parse(&b), Fetch::parse(&b));
        let _ = (Intent::parse(&b), Quote::parse(&b), parse_listing_reply(&b, &[1; 32]));
        let _ = (parse_chain_hash(&b), reply_body(&b, 7, 1), https_target(&b));
        let _ = (kind_of(10, &b), claim_ok(&b, &[1; 20]), approve_spender(&b));
        let _ = async_answer(&b, 42, 42, 7, 1);
        let words: Vec<[u8; 32]> = b.chunks_exact(32).map(|w| w.try_into().unwrap()).collect();
        let _ = (delivered(&words, &b), withdrawn(&words, &b));
    }
}

#[test]
fn what_parses_encodes_back_to_the_same_bytes() {
    for b in hostile() {
        if let Some(q) = Quote::parse(&b) {
            assert_eq!(q.encode().to_vec(), b, "Quote");
        }
        if let Some(g) = Gas::parse(&b) {
            assert_eq!(g.encode().to_vec(), b, "Gas");
        }
        if let Some(i) = Intent::parse(&b) {
            assert_eq!(i.encode(), Some(b.clone()), "Intent");
        }
        if let Some(r) = Read::parse(&b) {
            assert_eq!(r.encode(), Some(b.clone()), "Read");
        }
    }
}
