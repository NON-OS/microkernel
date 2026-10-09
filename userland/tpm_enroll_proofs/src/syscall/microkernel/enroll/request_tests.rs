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

//! What `MkEnroll` takes before the TPM is asked anything, and the challenge
//! libc frames, read back by the kernel.

use super::codec::{OP_ACTIVATE, OP_AK_PUBLIC, OP_AK_SIGN, OP_EK_CERTIFICATE, OP_EK_PUBLIC};
use super::request::{request, Request};
use crate::libc_enroll as libc;
use crate::security::tpm::enroll::{EkKind, ENCRYPTED_SECRET_MAX, ID_OBJECT_MAX};

#[test]
fn each_operation_takes_exactly_its_arguments() {
    assert_eq!(request(OP_EK_PUBLIC, 0, &[]), Some(Request::EkPublic(EkKind::Rsa2048)));
    assert_eq!(request(OP_EK_CERTIFICATE, 1, &[]), Some(Request::EkCertificate(EkKind::EccP256)));
    assert_eq!(request(OP_AK_PUBLIC, 0, &[]), Some(Request::AkPublic));
    assert_eq!(request(OP_AK_SIGN, 0, &[7; 32]), Some(Request::AkSign(&[7; 32])));
    let refused: [(u64, u64, &[u8]); 11] = [
        (OP_EK_PUBLIC, 2, &[]),
        (OP_EK_PUBLIC, 0, &[1]),
        (OP_EK_CERTIFICATE, 9, &[]),
        (OP_AK_PUBLIC, 1, &[]),
        (OP_AK_PUBLIC, 0, &[0]),
        (OP_AK_SIGN, 1, &[7; 32]),
        (OP_AK_SIGN, 0, &[7; 31]),
        (OP_AK_SIGN, 0, &[7; 33]),
        (OP_ACTIVATE, 2, &[1, 0, 9, 1, 0, 9]),
        (0, 0, &[]),
        (6, 0, &[]),
    ];
    for (op, kind, input) in refused {
        assert_eq!(request(op, kind, input), None, "op {op} kind {kind} input {input:?}");
    }
}

#[test]
fn a_challenge_libc_frames_is_the_one_the_kernel_reads() {
    for (b, s) in [(1, 1), (68, 256), (ID_OBJECT_MAX, ENCRYPTED_SECRET_MAX)] {
        let (blob, secret) = (vec![0xB1; b], vec![0x5E; s]);
        let mut frame = [0u8; libc::CHALLENGE_MAX];
        let n = libc::frame_challenge(&blob, &secret, &mut frame).expect("framed");
        let want = Some(Request::Activate(EkKind::EccP256, &blob, &secret));
        assert_eq!(request(OP_ACTIVATE, 1, &frame[..n]), want);
        for cut in 0..n {
            assert_eq!(request(OP_ACTIVATE, 1, &frame[..cut]), None, "cut at {cut}");
        }
        let longer = [&frame[..n], &[0]].concat();
        assert_eq!(request(OP_ACTIVATE, 1, &longer), None, "nothing after the secret");
    }
    let mut frame = [0u8; libc::CHALLENGE_MAX];
    assert_eq!(libc::frame_challenge(&[], &[1], &mut frame), None);
    assert_eq!(libc::frame_challenge(&[1], &[], &mut frame), None);
    assert_eq!(libc::frame_challenge(&[1; ID_OBJECT_MAX + 1], &[1], &mut frame), None);
    assert_eq!(libc::frame_challenge(&[1], &[1; ENCRYPTED_SECRET_MAX + 1], &mut frame), None);
}
