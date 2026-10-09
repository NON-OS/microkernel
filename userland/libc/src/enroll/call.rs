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

//! The five enrollment operations. Each returns the answer's length, or a
//! negative errno: EPERM without DeviceSecret, EINVAL for a refused input,
//! ENOENT when the TPM holds no certificate for that EK, EACCES when it
//! refused the command, ENODEV with no TPM, ENOMEM when `out` is too short.

use super::frame::*;
use crate::syscall::{call_raw, N_MK_ENROLL};

pub fn mk_enroll(op: u64, kind: u64, input: &[u8], out: &mut [u8]) -> i64 {
    let (ip, il) = (input.as_ptr() as u64, input.len() as u64);
    call_raw(N_MK_ENROLL, [op, kind, ip, il, out.as_mut_ptr() as u64, out.len() as u64])
}

/// The EK's name, then its TPM2B_PUBLIC; `split_public` reads it.
pub fn ek_public(kind: u64, out: &mut [u8; PUBLIC_MAX]) -> i64 {
    mk_enroll(OP_EK_PUBLIC, kind, &[], out)
}

/// The EK's certificate, DER, as the manufacturer wrote it.
pub fn ek_certificate(kind: u64, out: &mut [u8; CERT_MAX]) -> i64 {
    mk_enroll(OP_EK_CERTIFICATE, kind, &[], out)
}

/// The AK's name, then its TPM2B_PUBLIC.
pub fn ak_public(out: &mut [u8; PUBLIC_MAX]) -> i64 {
    mk_enroll(OP_AK_PUBLIC, 0, &[], out)
}

/// The secret in the registrar's challenge, made for `kind`'s EK and this AK.
/// The caller wipes `out` once it has answered the registrar.
pub fn activate(kind: u64, blob: &[u8], secret: &[u8], out: &mut [u8; SECRET_MAX]) -> i64 {
    let mut frame = [0u8; CHALLENGE_MAX];
    match frame_challenge(blob, secret, &mut frame) {
        Some(n) => mk_enroll(OP_ACTIVATE, kind, &frame[..n], out),
        None => -22, /* EINVAL */
    }
}

/// The AK's ECDSA P-256 signature over SHA-256 of `AK_SIGN_LABEL || msg`, r then s.
pub fn ak_sign(msg: &[u8; 32], out: &mut [u8; 64]) -> i64 {
    mk_enroll(OP_AK_SIGN, 0, msg, out)
}
