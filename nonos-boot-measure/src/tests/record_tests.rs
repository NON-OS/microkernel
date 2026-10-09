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

use sha2::{Digest, Sha256};

use super::record_build::{record, verifier};
use crate::record::{check, message, RecordError, P, RECORD_LEN};

const ROOT: [u8; 32] = [3u8; 32];

#[test]
fn the_message_is_the_domain_the_root_and_the_epoch() {
    let mut m = b"NONOS-BOOT-ROOT-v1".to_vec();
    m.extend(ROOT);
    m.extend(5u64.to_le_bytes());
    assert_eq!(message(&ROOT, 5), <[u8; 32]>::from(Sha256::digest(&m)));
}

#[test]
fn a_signed_record_at_or_above_the_floor_is_admitted() {
    for (epoch, floor) in [(5, 5), (6, 5), (0, 0)] {
        let rec = check(&record(ROOT, epoch), floor, verifier).expect("admitted");
        assert_eq!((rec.root, rec.epoch), (ROOT, epoch));
    }
}

#[test]
fn a_stale_epoch_is_refused_even_signed() {
    assert_eq!(check(&record(ROOT, 4), 5, verifier), Err(RecordError::Stale));
}

#[test]
fn any_changed_byte_is_refused() {
    let good = record(ROOT, 5);
    for i in 0..RECORD_LEN {
        let mut b = good.clone();
        b[i] ^= 0x01;
        let r = check(&b, 5, verifier);
        assert!(
            matches!(r, Err(RecordError::BadSignature | RecordError::Stale)),
            "byte {i}: {r:?}"
        );
    }
}

#[test]
fn a_non_canonical_root_word_or_a_wrong_length_is_refused() {
    let mut root = ROOT;
    root[8..16].copy_from_slice(&P.to_le_bytes());
    assert_eq!(check(&record(root, 5), 0, verifier), Err(RecordError::NonCanonicalRoot));
    root[8..16].copy_from_slice(&(P - 1).to_le_bytes());
    assert!(check(&record(root, 5), 0, verifier).is_ok());
    let good = record(ROOT, 5);
    assert_eq!(check(&good[..RECORD_LEN - 1], 0, verifier), Err(RecordError::Length));
    assert_eq!(check(&[good.clone(), vec![0]].concat(), 0, verifier), Err(RecordError::Length));
}
