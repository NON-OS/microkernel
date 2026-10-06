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

//! The kernel's check of a boot-root record's signature, against the record
//! the release tool wrote: `nonos-boot-measure/src/tests/fixture` holds one
//! signed by `tools/nonos-policy-approve boot-root` at epoch 7, and its
//! public point. The kernel verifies with its own P-256 over SHA-256 of
//! `NONOS-BOOT-ROOT-v1 || root || epoch`, as `loader_check::key` does; the
//! tool signed with Python's `cryptography`, so the two share no code.

use crate::crypto::asymmetric::p256::{verify, PublicKey, Signature};
use crate::hash::sha256::sha256;

const RECORD: &[u8; 104] =
    include_bytes!("../../../nonos-boot-measure/src/tests/fixture/boot_root.approval");
const PUB: &[u8; 64] =
    include_bytes!("../../../nonos-boot-measure/src/tests/fixture/boot_root.pub");

fn signed(rec: &[u8; 104]) -> bool {
    let mut msg = b"NONOS-BOOT-ROOT-v1".to_vec();
    msg.extend_from_slice(&rec[..40]);
    let mut pk: PublicKey = [0u8; 65];
    pk[0] = 0x04;
    pk[1..].copy_from_slice(PUB);
    let mut sig: Signature = [0u8; 64];
    sig.copy_from_slice(&rec[40..]);
    verify(&pk, &sha256(&msg), &sig)
}

#[test]
fn the_tools_record_verifies_under_the_kernels_p256() {
    assert_eq!(u64::from_le_bytes(RECORD[32..40].try_into().unwrap_or_default()), 7);
    assert!(signed(RECORD));
}

#[test]
fn any_changed_byte_of_the_record_is_refused() {
    for i in 0..RECORD.len() {
        let mut r = *RECORD;
        r[i] ^= 0x01;
        assert!(!signed(&r), "byte {i} changed and the record still verified");
    }
}
