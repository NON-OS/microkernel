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

//! The bytes libc reads out of `MkEnroll`'s answers, and the errno each
//! failure reaches it as, held to the kernel's.

use super::codec::*;
use super::errors::errno;
use crate::libc_enroll as libc;
use crate::security::tpm::enroll::{EnrollError, Public, AK_SIGN_LABEL, CERT_MAX, NAME_LEN};
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::KeyError;
use crate::syscall::microkernel::errnos::*;

#[test]
fn libc_and_the_kernel_agree_on_the_operations_and_bounds() {
    let ops = [OP_EK_PUBLIC, OP_EK_CERTIFICATE, OP_AK_PUBLIC, OP_ACTIVATE, OP_AK_SIGN];
    let libc_ops = [
        libc::OP_EK_PUBLIC,
        libc::OP_EK_CERTIFICATE,
        libc::OP_AK_PUBLIC,
        libc::OP_ACTIVATE,
        libc::OP_AK_SIGN,
    ];
    assert_eq!(ops, libc_ops);
    assert_eq!((libc::CHALLENGE_MAX, libc::NAME_LEN), (INPUT_MAX, NAME_LEN));
    assert_eq!(libc::CERT_MAX, CERT_MAX, "a certificate the kernel reads fits");
    assert_eq!(
        libc::AK_SIGN_LABEL,
        AK_SIGN_LABEL,
        "the registrar verifies over the kernel's label"
    );
}

#[test]
fn libc_splits_the_public_answer_the_kernel_writes() {
    let p = Public { area: vec![0, 4, 1, 2, 3, 4], name: [9; NAME_LEN] };
    let bytes = public_bytes(&p);
    assert_eq!(libc::split_public(&bytes), Some((&p.name, &p.area[..])));
    assert_eq!(libc::split_public(&bytes[..bytes.len() - 1]), None);
    assert_eq!(libc::split_public(&[&bytes[..], &[0]].concat()), None);
}

#[test]
fn each_failure_has_its_errno() {
    let tpm = |e| EnrollError::Key(KeyError::Tpm(e));
    let refused = |rc| EnrollError::Key(KeyError::Refused(rc));
    assert_eq!(errno(tpm(TpmError::NotPresent)), ERRNO_NODEV);
    assert_eq!(errno(tpm(TpmError::Timeout)), ERRNO_TIMEDOUT);
    assert_eq!(errno(tpm(TpmError::InvalidResponse)), ERRNO_IO);
    assert_eq!(errno(refused(0x18B)), ERRNO_NOENT, "no certificate at the index");
    assert_eq!(errno(refused(0x1DF)), ERRNO_ACCES, "a credential for another key");
    assert_eq!(errno(EnrollError::OutOfBounds), ERRNO_INVAL);
    assert_eq!(errno(EnrollError::Unsignable), ERRNO_INVAL);
}
