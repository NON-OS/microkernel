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


//! The SubjectPublicKeyInfo of a certificate, found by walking the DER
//! (RFC 5280 4.1): Certificate, then TBSCertificate, past the optional
//! version and the five fields before it.

/// One TLV at `at`: its tag, where its value starts, and where it ends.
fn tlv(der: &[u8], at: usize) -> Option<(u8, usize, usize)> {
    let tag = *der.get(at)?;
    let first = *der.get(at + 1)?;
    let (len, start) = if first < 0x80 {
        (usize::from(first), at + 2)
    } else {
        let count = usize::from(first & 0x7f);
        if count == 0 || count > 3 {
            return None;
        }
        let mut len = 0usize;
        for i in 0..count {
            len = len << 8 | usize::from(*der.get(at + 2 + i)?);
        }
        (len, at + 2 + count)
    };
    let end = start.checked_add(len)?;
    (end <= der.len()).then_some((tag, start, end))
}

/// The SubjectPublicKeyInfo, whole, as the crypto pool's RSA verify takes
/// it. `None` for anything that is not a certificate.
pub fn spki(cert: &[u8]) -> Option<&[u8]> {
    let (tag, value, end) = tlv(cert, 0)?;
    if tag != 0x30 || end != cert.len() {
        return None;
    }
    let (tag, tbs, tbs_end) = tlv(cert, value)?;
    if tag != 0x30 {
        return None;
    }
    let mut at = tbs;
    if cert.get(at) == Some(&0xa0) {
        at = tlv(cert, at)?.2;
    }
    // serial, signature algorithm, issuer, validity, subject
    for _ in 0..5 {
        let (_, _, next) = tlv(cert, at)?;
        if next > tbs_end {
            return None;
        }
        at = next;
    }
    let (tag, _, spki_end) = tlv(cert, at)?;
    (tag == 0x30 && spki_end <= tbs_end).then(|| &cert[at..spki_end])
}
