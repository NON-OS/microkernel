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

//! The AK signs a 32-byte message through Hash and Sign, p256 accepts the
//! signature against the AK's public area, and the TPM holds the key to that:
//! no ticket, no signature. What it signs is the enrollment label and the
//! message, so no message makes the signature pass for an attestation.

use sha2::{Digest, Sha256};

use super::steps::{load_test_ak, sign};
use super::swtpm::Swtpm;
use super::verify::{labelled, verifies};
use crate::security::tpm::enroll::sign::{build_sign, parse_sign};
use crate::security::tpm::enroll::EnrollError;
use crate::security::tpm::machine_key::run::run;
use crate::security::tpm::machine_key::KeyError;

#[test]
fn the_ak_signature_verifies_against_its_public_area() {
    let Some(_t) = Swtpm::start("the_ak_signature_verifies_against_its_public_area") else {
        return;
    };
    let (ak, public) = load_test_ak();
    let msg = *b"commit(s) for this device, 32 by";
    let (digest, sig) = sign(ak, &msg).expect("hash and sign");
    assert_eq!(digest[..], Sha256::digest(labelled(&msg))[..], "the TPM hashed label and message");
    assert!(verifies(&public.area, &labelled(&msg), &sig), "p256 accepts the signature");
    assert!(!verifies(&public.area, &msg, &sig), "never over the bare message");
    let mut other = msg;
    other[31] ^= 1;
    assert!(!verifies(&public.area, &labelled(&other), &sig), "over that message only");
    let (_, again) = sign(ak, &msg).expect("second signature");
    assert!(verifies(&public.area, &labelled(&msg), &again));
}

#[test]
fn a_message_shaped_like_an_attestation_is_signed_only_under_the_label() {
    let test = "a_message_shaped_like_an_attestation_is_signed_only_under_the_label";
    let Some(_t) = Swtpm::start(test) else { return };
    let (ak, public) = load_test_ak();
    let mut msg = [0x11u8; 32];
    msg[..4].copy_from_slice(&0xFF54_4347u32.to_be_bytes());
    let (_, sig) = sign(ak, &msg).expect("signed: the hashed bytes open with the label");
    assert!(verifies(&public.area, &labelled(&msg), &sig));
    assert!(!verifies(&public.area, &msg, &sig), "never as the attestation-shaped bytes");
}

#[test]
fn the_ak_will_not_sign_a_digest_the_tpm_did_not_hash() {
    let Some(_t) = Swtpm::start("the_ak_will_not_sign_a_digest_the_tpm_did_not_hash") else {
        return;
    };
    let (ak, _) = load_test_ak();
    let digest: [u8; 32] = Sha256::digest([0x22u8; 32]).into();
    /* TPM_ST_HASHCHECK, TPM_RH_NULL, empty digest: the null ticket */
    let null = [0x80, 0x24, 0x40, 0x00, 0x00, 0x07, 0x00, 0x00];
    let r = parse_sign(&run(&build_sign(ak, &digest, &null)).expect("sent"));
    let Err(EnrollError::Key(KeyError::Refused(rc))) = r else { panic!("signed: {r:?}") };
    assert_eq!(rc & 0x3F, 0x20, "TPM_RC_TICKET, got {rc:#x}");
}
