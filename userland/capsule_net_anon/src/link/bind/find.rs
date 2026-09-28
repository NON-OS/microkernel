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

//! Picking the two certificates the chain needs out of a CERTS cell.

use super::super::certs::split;
use super::super::constants::{CERT_ED_ID_SIGN, CERT_ED_SIGN_LINK};
use super::super::ed_cert::{parse, EdCert};
use super::error::BindError;

pub(super) fn find(certs: &[u8]) -> Result<(EdCert<'_>, EdCert<'_>), BindError> {
    let mut signing = None;
    let mut link = None;
    for entry in split(certs).ok_or(BindError::Malformed)? {
        let Some(cert) = parse(entry.body) else { continue };
        if entry.cert_type != cert.cert_type {
            continue;
        }
        match entry.cert_type {
            CERT_ED_ID_SIGN => signing = Some(cert),
            CERT_ED_SIGN_LINK => link = Some(cert),
            _ => {}
        }
    }
    Ok((signing.ok_or(BindError::MissingSigningCert)?, link.ok_or(BindError::MissingLinkCert)?))
}
