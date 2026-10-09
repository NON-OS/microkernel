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

//! Reading the fixed part of a certificate, cert-spec section 2.1.

use super::super::constants::{CERT_VERSION, SIGNATURE_BYTES};
use super::extension::signing_key;
use super::types::EdCert;

const FIXED: usize = 1 + 1 + 4 + 1 + 32 + 1;

const N_EXTENSIONS_AT: usize = 39;

/// Parse one certificate. `None` on a length that does not add up, an unexpected
/// version, or a truncated extension.
pub fn parse(body: &[u8]) -> Option<EdCert<'_>> {
    if body.len() < FIXED + SIGNATURE_BYTES || body[0] != CERT_VERSION {
        return None;
    }
    let expiry_hours = u32::from_be_bytes([body[2], body[3], body[4], body[5]]);
    let mut certified_key = [0u8; 32];
    certified_key.copy_from_slice(&body[7..39]);

    let signed_end = body.len() - SIGNATURE_BYTES;
    let mut signature = [0u8; SIGNATURE_BYTES];
    signature.copy_from_slice(&body[signed_end..]);
    Some(EdCert {
        cert_type: body[1],
        expiry_hours,
        certified_key,
        signed_with: signing_key(body.get(FIXED..signed_end)?, body[N_EXTENSIONS_AT]),
        signed: &body[..signed_end],
        signature,
    })
}
