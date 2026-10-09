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

//! A request run against the TPM. The activated secret stays in the buffer
//! that wipes itself until it is copied out to the caller.

use alloc::vec::Vec;

use super::codec::public_bytes;
use super::request::Request;
use crate::security::tpm::enroll::{
    activate_credential, ak_public, ak_sign, ek_certificate, ek_public, Activated, EnrollError,
};

pub(super) enum Answer {
    Bytes(Vec<u8>),
    Secret(Activated),
}

impl Answer {
    pub(super) fn bytes(&self) -> &[u8] {
        match self {
            Answer::Bytes(b) => b,
            Answer::Secret(s) => s.as_bytes(),
        }
    }
}

pub(super) fn answer(r: Request<'_>) -> Result<Answer, EnrollError> {
    Ok(match r {
        Request::EkPublic(kind) => Answer::Bytes(public_bytes(&ek_public(kind)?)),
        Request::EkCertificate(kind) => Answer::Bytes(ek_certificate(kind)?),
        Request::AkPublic => Answer::Bytes(public_bytes(&ak_public()?)),
        Request::Activate(kind, blob, secret) => {
            Answer::Secret(activate_credential(kind, blob, secret)?)
        }
        Request::AkSign(msg) => Answer::Bytes(ak_sign(msg)?.to_vec()),
    })
}
