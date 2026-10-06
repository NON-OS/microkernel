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

//! OP_LOAD_INDEX hands its whole body to decode_index before any signature
//! is checked, so the decode reads bytes nobody has vouched for yet. Seeded
//! from blobs the abi's own encoder writes, then damaged where the length
//! and count fields live, every input either decodes within the abi's caps,
//! with the signed bytes a prefix of the blob, or is refused; and a count past
//! a cap is refused before anything is reserved for it.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use nonos_marketplace_abi::{
    decode_index, encode_and_sign, DecodeError, MarketplaceIndex, MAX_ARCHES, MAX_CAPABILITIES,
    MAX_ENTRIES, MAX_PUBLISHER, MAX_RELEASES, MAX_SIGNATURE,
};

use crate::release_tests::index;

const FUZZ_ROUNDS: usize = 200_000;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn blob_of(index: MarketplaceIndex) -> Vec<u8> {
    encode_and_sign(index, |_| [7; 64]).0
}

/// Indexes of zero, one and three listings, encoded as the operator would.
fn seeds() -> Vec<Vec<u8>> {
    let mut empty = index();
    empty.entries.clear();
    let mut three = index();
    let mut second = three.entries[0].clone();
    second.listing_id = String::from("linux.yq");
    second.releases[0].supported_arches.push(String::from("aarch64-linux"));
    second.releases[0].required_capabilities = vec![String::from("net"), String::from("fs")];
    let mut third = second.clone();
    third.listing_id = String::from("linux.zq");
    third.releases.clear();
    three.entries.push(second);
    three.entries.push(third);
    vec![blob_of(empty), blob_of(index()), blob_of(three)]
}

/// What must hold for any blob.
fn check(blob: &[u8]) -> bool {
    let Ok(decoded) = decode_index(blob) else { return false };
    assert!(blob.starts_with(decoded.signed_bytes), "the signed bytes are the blob's head");
    assert!(decoded.signed_bytes.len() + 4 + decoded.index.index_signature.len() <= blob.len());
    assert!(decoded.index.index_signature.len() <= MAX_SIGNATURE as usize);
    assert!(decoded.index.operator_id.len() <= MAX_PUBLISHER as usize);
    assert!(decoded.index.entries.len() <= MAX_ENTRIES as usize);
    for e in &decoded.index.entries {
        assert!(e.releases.len() <= MAX_RELEASES as usize);
        for r in &e.releases {
            assert!(r.supported_arches.len() <= MAX_ARCHES as usize);
            assert!(r.required_capabilities.len() <= MAX_CAPABILITIES as usize);
        }
    }
    true
}

#[test]
fn the_encoders_blobs_decode_back() {
    for (blob, listings) in seeds().iter().zip([0usize, 1, 3]) {
        assert!(check(blob));
        let Ok(d) = decode_index(blob) else { panic!("an encoded index decodes") };
        assert_eq!(d.index.entries.len(), listings);
        assert_eq!(d.index.index_signature, vec![7u8; 64]);
        assert_eq!(d.signed_bytes.len(), blob.len() - 4 - 64);
    }
}

#[test]
fn damaged_blobs_decode_within_the_caps_or_not_at_all() {
    let seeds = seeds();
    let mut s = 0x4E4D_4B54_0000_0001u64;
    let mut decoded = 0usize;
    for _ in 0..FUZZ_ROUNDS {
        let r = xorshift(&mut s);
        let mut b = seeds[(r % 3) as usize].clone();
        for _ in 0..1 + (r >> 8) % 3 {
            let at = (xorshift(&mut s) % b.len() as u64) as usize;
            match xorshift(&mut s) % 6 {
                // A length or count field: small, at a cap, past it, or huge.
                0 | 1 => {
                    let v: u32 = match xorshift(&mut s) % 6 {
                        0 => (xorshift(&mut s) % 8) as u32,
                        1 => MAX_ENTRIES,
                        2 => MAX_ENTRIES + 1,
                        3 => MAX_RELEASES + (xorshift(&mut s) % 2) as u32,
                        4 => u32::MAX - (xorshift(&mut s) % 4) as u32,
                        _ => xorshift(&mut s) as u32,
                    };
                    let end = (at + 4).min(b.len());
                    b[at..end].copy_from_slice(&v.to_le_bytes()[..end - at]);
                }
                2 => b[at] ^= 1 << (xorshift(&mut s) % 8),
                3 => b.truncate(at),
                4 => b.insert(at, xorshift(&mut s) as u8),
                _ => {
                    b.remove(at);
                }
            }
            if b.is_empty() {
                break;
            }
        }
        if check(&b) {
            decoded += 1;
        }
    }
    assert!(decoded > FUZZ_ROUNDS / 50, "the generator reaches whole decodes: {decoded}");
}

#[test]
fn boundary_blobs() {
    let good = &seeds()[1];
    assert_eq!(decode_index(&[]).err(), Some(DecodeError::Short));
    assert_eq!(decode_index(&good[..3]).err(), Some(DecodeError::Short));
    // Every prefix of a good blob is refused, not read past its end.
    for n in 0..good.len() {
        assert!(decode_index(&good[..n]).is_err(), "a prefix of {n} bytes");
    }
    // Bytes after the signature are left alone.
    let mut longer = good.clone();
    longer.push(0);
    assert!(check(&longer));
    // The operator id's length one past its bytes, and at its maximum.
    let at = 4;
    for v in [u32::from_le_bytes(good[at..at + 4].try_into().unwrap_or([0; 4])) + 200, u32::MAX] {
        let mut b = good.clone();
        b[at..at + 4].copy_from_slice(&v.to_le_bytes());
        assert!(decode_index(&b).is_err(), "operator id length {v}");
    }
    // An entry count past the cap is refused before anything is reserved for
    // it; at the cap with no bytes behind it, the reads run short.
    let count_at = 4 + 4 + "nonos.marketplace.v1".len() + 32 + 8 + 8;
    for (count, want) in [
        (MAX_ENTRIES + 1, DecodeError::TooManyItems),
        (u32::MAX, DecodeError::TooManyItems),
        (MAX_ENTRIES, DecodeError::Short),
    ] {
        let mut b = good[..count_at].to_vec();
        b.extend_from_slice(&count.to_le_bytes());
        assert_eq!(decode_index(&b).err(), Some(want), "count {count}");
    }
    // A blob past the size cap is refused whole.
    let huge = vec![0u8; nonos_marketplace_abi::limits::MAX_INDEX_BLOB + 1];
    assert_eq!(decode_index(&huge).err(), Some(DecodeError::BlobTooLarge));
}
