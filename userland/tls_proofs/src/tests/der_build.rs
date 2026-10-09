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

//! Building certificates by hand, for the shapes no CA would issue on request.
//!
//! The structure is a real X.509 v3 layout, so the parsers walk it the way they
//! walk a served certificate. The signature is a placeholder: these feed the
//! name and structure checks, which run before and apart from any signature.

/// One DER element, short or long form length as the content needs.
pub(crate) fn tlv(tag: u8, content: &[u8]) -> Vec<u8> {
    let n = content.len();
    let mut out = vec![tag];
    if n < 0x80 {
        out.push(n as u8);
    } else if n < 0x100 {
        out.extend_from_slice(&[0x81, n as u8]);
    } else {
        out.extend_from_slice(&[0x82, (n >> 8) as u8, n as u8]);
    }
    out.extend_from_slice(content);
    out
}

pub(crate) fn seq(parts: &[&[u8]]) -> Vec<u8> {
    tlv(0x30, &parts.concat())
}

/// A GeneralNames SEQUENCE out of (tag, value) pairs; 0x82 is a dNSName.
pub(crate) fn general_names(names: &[(u8, &[u8])]) -> Vec<u8> {
    let parts: Vec<Vec<u8>> = names.iter().map(|(tag, value)| tlv(*tag, value)).collect();
    tlv(0x30, &parts.concat())
}

/// A v3 certificate whose RSA key is `key` and whose only extension is a
/// subjectAltName holding `san` (a GeneralNames SEQUENCE).
pub(crate) fn certificate(key: &[u8], san: &[u8]) -> Vec<u8> {
    let sha256_rsa = [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x0B];
    let rsa = [0x2A, 0x86, 0x48, 0x86, 0xF7, 0x0D, 0x01, 0x01, 0x01];
    let version = tlv(0xa0, &tlv(0x02, &[2]));
    let serial = tlv(0x02, &[1]);
    let alg = seq(&[&tlv(0x06, &sha256_rsa), &tlv(0x05, &[])]);
    let rdn = seq(&[&tlv(0x06, &[0x55, 0x04, 0x03]), &tlv(0x0c, b"proof")]);
    let name = seq(&[&tlv(0x31, &rdn)]);
    let validity = seq(&[&tlv(0x17, b"260101000000Z"), &tlv(0x17, b"270101000000Z")]);
    let mut bits = vec![0u8];
    bits.extend_from_slice(key);
    let spki = seq(&[&seq(&[&tlv(0x06, &rsa), &tlv(0x05, &[])]), &tlv(0x03, &bits)]);
    let ext = seq(&[&tlv(0x06, &[0x55, 0x1d, 0x11]), &tlv(0x04, san)]);
    let exts = tlv(0xa3, &seq(&[&ext]));
    let tbs = seq(&[&version, &serial, &alg, &name, &validity, &name, &spki, &exts]);
    seq(&[&tbs, &alg, &tlv(0x03, &[0, 1, 2, 3])])
}

/// RSA key bytes the requester chooses freely, carrying `inner` somewhere in
/// the middle. A modulus can be ground to hold chosen bytes in its top half,
/// and a CA signs whatever key the requester sends.
pub(crate) fn key_holding(inner: &[u8]) -> Vec<u8> {
    let mut key = vec![0xC3u8; 40];
    key.extend_from_slice(inner);
    key.extend_from_slice(&[0x5Au8; 40]);
    key
}
