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

use alloc::vec;

use nonos_marketplace_abi::{decode_index, DecodeError, MAX_ENTRIES};

use super::seeds::seeds;

#[test]
fn boundary_blobs() {
    let good = &seeds()[1];
    assert_eq!(decode_index(&[]).err(), Some(DecodeError::Short));
    assert_eq!(decode_index(&good[..3]).err(), Some(DecodeError::Short));
    // Every prefix of a good blob is refused, not read past its end.
    for n in 0..good.len() {
        assert!(decode_index(&good[..n]).is_err(), "a prefix of {n} bytes");
    }
    // Bytes after the signature are signed by nobody, so they are refused.
    let mut longer = good.clone();
    longer.push(0);
    assert_eq!(decode_index(&longer).err(), Some(DecodeError::Trailing));
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
