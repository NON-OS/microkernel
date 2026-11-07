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

//! The record as it is kept on disk.

use nonos_lock::blob::{frame, split, Kind, BLOB_MAX, KIND_PLAIN, KIND_SEALED};
use nonos_lock::RECORD_LEN;

#[test]
fn each_kind_frames_and_splits_back() {
    let mut out = [0u8; BLOB_MAX];
    let rec = [3u8; RECORD_LEN];
    let n = frame(Kind::Plain, &rec, &mut out).unwrap();
    assert_eq!(out[0], KIND_PLAIN);
    assert_eq!(split(&out[..n]), Some((Kind::Plain, &rec[..])));
    let sealed = [5u8; 100];
    let n = frame(Kind::Sealed, &sealed, &mut out).unwrap();
    assert_eq!(out[0], KIND_SEALED);
    assert_eq!(split(&out[..n]), Some((Kind::Sealed, &sealed[..])));
}

#[test]
fn malformed_blobs_are_refused() {
    assert_eq!(split(&[]), None);
    assert_eq!(split(&[KIND_SEALED]), None);
    assert_eq!(split(&[KIND_PLAIN, 1, 2, 3]), None);
    assert_eq!(split(&[9, 1, 2, 3]), None);
    let mut long = vec![KIND_SEALED; BLOB_MAX + 1];
    long[0] = KIND_SEALED;
    assert_eq!(split(&long), None);
    let mut small = [0u8; 4];
    assert_eq!(frame(Kind::Sealed, &[1u8; 10], &mut small), None);
}
