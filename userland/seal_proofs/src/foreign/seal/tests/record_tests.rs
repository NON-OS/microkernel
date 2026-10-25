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

//! The kernel's record against plan/11's offsets, read back by hand.

use crate::foreign::seal::record::{encode, Facts, FINAL, SATURATED};

fn facts() -> Facts {
    Facts {
        flags: FINAL | SATURATED,
        sup_meas: [1; 32],
        sup_caps: 0x3_0400_1979,
        authority: 255,
        seal_no: 0x0102_0304_0506_0708,
        trace: [2; 32],
        rows: 1 << 40,
        hc: [3, 4, 5, 0xFFFF_FFFF_0000_0000],
        meas: [6; 32],
        n_meas: 3,
        app_id: [7; 32],
    }
}

fn u64_at(b: &[u8], at: usize) -> u64 {
    u64::from_le_bytes(b[at..at + 8].try_into().unwrap())
}

#[test]
fn every_field_sits_at_its_offset() {
    let f = facts();
    let b = encode(&f);
    assert_eq!((&b[..4], &b[4..6], &b[6..8]), (&b"NFSR"[..], &[1, 0][..], &[3, 0][..]));
    assert_eq!((&b[8..40], u64_at(&b, 40), b[48]), (&f.sup_meas[..], f.sup_caps, 255));
    assert_eq!((u64_at(&b, 56), &b[64..96], u64_at(&b, 96)), (f.seal_no, &f.trace[..], f.rows));
    let hc: Vec<u64> = (0..4).map(|i| u64_at(&b, 104 + 8 * i)).collect();
    assert_eq!((hc, &b[136..168], &b[168..172]), (f.hc.to_vec(), &f.meas[..], &[3, 0, 0, 0][..]));
    assert_eq!(&b[176..208], &f.app_id[..]);
}

#[test]
fn the_reserved_bytes_and_the_diag_slot_stay_zero() {
    let b = encode(&facts());
    for (lo, hi) in [(49, 56), (172, 176), (208, 240)] {
        assert!(b[lo..hi].iter().all(|&x| x == 0), "{lo}..{hi}");
    }
}

#[test]
fn the_verifiers_decoder_reads_the_kernels_record() {
    let f = facts();
    let r = nonos_proof_abi::decode_record(&encode(&f)).expect("a record the verifier reads");
    assert_eq!((r.flags, r.authority, r.seal_no, r.rows), (f.flags, 255, f.seal_no, f.rows));
    assert_eq!((r.sup_meas, r.trace, r.hc, r.meas), (f.sup_meas, f.trace, f.hc, f.meas));
    assert_eq!((r.sup_caps, r.n_meas, r.app_id), (f.sup_caps, f.n_meas, f.app_id));
}
