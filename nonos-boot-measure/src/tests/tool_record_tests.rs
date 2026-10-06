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

//! The record as the release tool writes it: `fixture/boot_root.approval` was
//! signed by `tools/nonos-policy-approve boot-root` at epoch 7 over the fixture
//! root, under a throwaway P-256 key whose public point is `boot_root.pub`. The
//! private half was deleted; the tool and this crate share no code.

use p256::ecdsa::signature::hazmat::PrehashVerifier;
use p256::ecdsa::{Signature, VerifyingKey};

use super::gate_build::{booted, root, TRAILER};
use super::record_build::verifier;
use crate::gate::measured;
use crate::record::{check, RecordError};

const RECORD: &[u8] = include_bytes!("fixture/boot_root.approval");
const PUB: &[u8; 64] = include_bytes!("fixture/boot_root.pub");

fn release(d: &[u8; 32], r: &[u8; 32], s: &[u8; 32]) -> bool {
    let sec1 = [&[0x04u8][..], PUB].concat();
    let (Ok(key), Ok(sig)) =
        (VerifyingKey::from_sec1_bytes(&sec1), Signature::from_scalars(*r, *s))
    else {
        return false;
    };
    key.verify_prehash(d, &sig).is_ok()
}

#[test]
fn the_tools_record_verifies_here_and_admits_the_measured_loader() {
    let rec = check(RECORD, 7, release).expect("the tool's record verifies");
    assert_eq!((rec.root, rec.epoch), (root(), 7));
    let (log, pcr) = booted(&[]);
    let a = measured(&log, &pcr, 7, RECORD, TRAILER, release).expect("admitted");
    assert_eq!(a.epoch, 7);
}

#[test]
fn the_tools_record_is_stale_above_its_epoch_and_foreign_under_another_key() {
    assert_eq!(check(RECORD, 8, release), Err(RecordError::Stale));
    assert_eq!(check(RECORD, 0, verifier), Err(RecordError::BadSignature));
}
