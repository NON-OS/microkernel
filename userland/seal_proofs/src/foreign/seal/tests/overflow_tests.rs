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

//! An overflowed family seals final with the overflow flag, and the verifier
//! reads that record field for field before refusing it.

use nonos_proof_abi::{decode_record, FLAG_FINAL, FLAG_OVERFLOW};

use super::fixture::family;
use crate::foreign::seal::family::MAX_MEAS;
use crate::foreign::seal::finish::finish;
use crate::foreign::seal::table::{LIVE, SEALED};

#[test]
fn an_overflowed_family_is_flagged_in_its_record() {
    let mut f = family(79);
    f.app_id = Some([0xA2; 32]);
    (0..=MAX_MEAS).for_each(|i| {
        let mut m = [0u8; 32];
        m[..8].copy_from_slice(&(i as u64).to_le_bytes());
        f.measured(&m);
    });
    LIVE.lock().push(f);
    let n = finish(79).expect("a seal");
    let sealed = SEALED.lock();
    let s = sealed.iter().find(|s| s.seal_no == n).expect("kept");
    let r = decode_record(&s.record).expect("the verifier reads it");
    assert_eq!((r.flags, r.n_meas as usize), (FLAG_FINAL | FLAG_OVERFLOW, MAX_MEAS));
    assert_eq!(s.bytes_len(), s.bytes().len());
}
