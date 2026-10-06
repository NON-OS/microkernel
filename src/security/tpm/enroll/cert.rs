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

//! The EK certificate, out of the NV index the profile assigns it.

use alloc::vec::Vec;

use super::consts::NV_CHUNK;
use super::error::EnrollError;
use super::kind::EkKind;
use super::nv_public::{build_nv_read_public, parse_nv_read_public};
use super::nv_read::{build_nv_read, parse_nv_read};
use crate::security::tpm::error::TpmError;
use crate::security::tpm::machine_key::run::run;

/// The DER certificate for `kind`'s EK, as the manufacturer wrote it. The
/// TPM refuses with `TPM_RC_HANDLE` when it carries none for this kind.
pub fn ek_certificate(kind: EkKind) -> Result<Vec<u8>, EnrollError> {
    let index = kind.cert_index();
    let slot = parse_nv_read_public(&run(&build_nv_read_public(index))?, index)?;
    let mut out = Vec::with_capacity(slot.size);
    while out.len() < slot.size {
        let n = (slot.size - out.len()).min(NV_CHUNK);
        let resp = run(&build_nv_read(slot.auth, index, n as u16, out.len() as u16))?;
        out.extend_from_slice(parse_nv_read(&resp, n)?);
    }
    der_trim(out)
}

/// The certificate without the fill some parts leave after it in the index:
/// the length its own DER SEQUENCE header gives, which must fit in what was
/// read. Anything that does not open with such a header is not a certificate.
pub(in crate::security::tpm) fn der_trim(mut data: Vec<u8>) -> Result<Vec<u8>, EnrollError> {
    let len = match data.get(..4) {
        Some(&[0x30, 0x82, hi, lo]) => 4 + (usize::from(hi) << 8 | usize::from(lo)),
        Some(&[0x30, 0x81, n, _]) => 3 + usize::from(n),
        _ => return Err(TpmError::InvalidResponse.into()),
    };
    if len > data.len() {
        return Err(TpmError::InvalidResponse.into());
    }
    data.truncate(len);
    Ok(data)
}
