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

//! The registrar's credential, made by tpm2_makecredential to the kernel's EK
//! public area and the kernel's AK name, unwraps to the registrar's secret.
//! Made for any other name, or altered, it does not. Nothing the sequence
//! loaded stays loaded either way.

use super::steps::{activate, load_test_ak};
use super::swtpm::Swtpm;
use super::tools::{held, make_credential};
use crate::security::tpm::enroll::ek_calls::derive_ek_public as ek_public;
use crate::security::tpm::enroll::{EkKind, EnrollError};
use crate::security::tpm::machine_key::KeyError;

fn round_trip(kind: EkKind, test: &str) {
    let Some(t) = Swtpm::start(test) else { return };
    let (ak, ak_pub) = load_test_ak();
    let ek = ek_public(kind).expect("ek public");
    let secret = *b"the registrar chose these bytes.";
    let (blob, enc) = make_credential(&t.dir, &ek.area, &ak_pub.name, &secret);
    let got = activate(ak, kind, &blob, &enc).expect("activation");
    assert_eq!(got.as_bytes(), &secret[..]);
    assert_eq!(held(&t), (1, 0), "only the AK stays loaded");
    eprintln!("{test}: {} byte blob, {} byte secret, unwrapped", blob.len(), enc.len());
}

#[test]
fn rsa_ek_activation_returns_the_registrars_secret() {
    round_trip(EkKind::Rsa2048, "rsa_ek_activation_returns_the_registrars_secret");
}

#[test]
fn ecc_ek_activation_returns_the_registrars_secret() {
    round_trip(EkKind::EccP256, "ecc_ek_activation_returns_the_registrars_secret");
}

fn refused(r: Result<impl Sized, EnrollError>) -> u32 {
    match r {
        Err(EnrollError::Key(KeyError::Refused(rc))) => rc,
        Err(e) => panic!("expected a TPM refusal, got {e:?}"),
        Ok(_) => panic!("activated a credential not made for this AK"),
    }
}

#[test]
fn a_credential_for_another_name_or_altered_is_refused() {
    let Some(t) = Swtpm::start("a_credential_for_another_name_or_altered_is_refused") else {
        return;
    };
    let (ak, ak_pub) = load_test_ak();
    let ek = ek_public(EkKind::Rsa2048).expect("ek public");
    let mut other = ak_pub.name;
    other[33] ^= 1;
    let (blob, enc) = make_credential(&t.dir, &ek.area, &other, &[7; 32]);
    let rc = refused(activate(ak, EkKind::Rsa2048, &blob, &enc));
    assert_eq!(rc & 0x3F, 0x1F, "TPM_RC_INTEGRITY, got {rc:#x}");
    let (mut blob, enc) = make_credential(&t.dir, &ek.area, &ak_pub.name, &[7; 32]);
    blob[40] ^= 1;
    let rc2 = refused(activate(ak, EkKind::Rsa2048, &blob, &enc));
    assert_eq!(held(&t), (1, 0), "EK and session flushed on the refused paths");
    eprintln!("other name refused with {rc:#x}, altered blob with {rc2:#x}");
}
