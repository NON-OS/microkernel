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

//! `MkEnroll` end to end but for user memory: libc frames each call, the
//! kernel's `request` reads it, `answer` runs it through `load_ak` and
//! `machine_key::run`, and libc reads what comes back. The only test that goes
//! through `load_ak`, whose handle the process keeps once, as the kernel keeps
//! it once per boot.

use super::answer::answer;
use super::request::request;
use crate::libc_enroll::{
    frame_challenge, split_public, CHALLENGE_MAX, EK_ECC_P256, EK_RSA2048, OP_ACTIVATE,
    OP_AK_PUBLIC, OP_AK_SIGN, OP_EK_PUBLIC, PUBLIC_MAX, SECRET_MAX,
};
use crate::security::tpm::live::steps::fresh_ek;
use crate::security::tpm::enroll::{EkKind, EnrollError};
use crate::security::tpm::live::swtpm::Swtpm;
use crate::security::tpm::live::tools::{held, make_credential};
use crate::security::tpm::live::verify::{labelled, verifies};

/// One call as the kernel takes it, answered as user memory receives it.
fn call(op: u64, kind: u64, input: &[u8]) -> Result<Vec<u8>, EnrollError> {
    let req = request(op, kind, input).expect("a request the kernel takes");
    answer(req).map(|a| a.bytes().to_vec())
}

#[test]
fn enrollment_end_to_end_through_the_syscall_framing() {
    let test = "enrollment_end_to_end_through_the_syscall_framing";
    let Some(t) = Swtpm::start(test) else { return };
    let ak = call(OP_AK_PUBLIC, 0, &[]).expect("ak public");
    let (ak_name, ak_area) = split_public(&ak).expect("name and area");
    assert!(ak.len() <= PUBLIC_MAX, "fits libc's buffer");
    for (kind, ek_kind) in [(EK_ECC_P256, EkKind::EccP256), (EK_RSA2048, EkKind::Rsa2048)] {
        let ek = call(OP_EK_PUBLIC, kind, &[]).expect("ek public");
        let (ek_name, ek_area) = split_public(&ek).expect("name and area");
        assert!(ek.len() <= PUBLIC_MAX, "{ek_kind:?} fits libc's buffer");
        let direct = fresh_ek(ek_kind).expect("ek");
        assert_eq!((&direct.name, &direct.area[..]), (ek_name, ek_area), "the entry point's");
        let secret = [0x5A; 32];
        let (blob, enc) = make_credential(&t.dir, ek_area, &ak_name[..], &secret);
        let mut frame = [0u8; CHALLENGE_MAX];
        let n = frame_challenge(&blob, &enc, &mut frame).expect("framed");
        let got = call(OP_ACTIVATE, kind, &frame[..n]).expect("activated");
        assert!(got.len() <= SECRET_MAX && got == secret, "the registrar's secret");
        let empty = call(OP_ACTIVATE, kind, &[1, 0, 0xB1, 0, 0]).err();
        assert_eq!(empty, Some(EnrollError::OutOfBounds), "bounded before the TPM");
    }
    let msg = [0x33; 32];
    let sig = call(OP_AK_SIGN, 0, &msg).expect("signature");
    assert!(verifies(ak_area, &labelled(&msg), &sig.try_into().expect("r then s")));
    assert_eq!(held(&t), (1, 0), "only the AK, as after bring-up");
}
