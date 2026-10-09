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

// id-ce-subjectAltName, 2.5.29.17.
const OID_SUBJECT_ALT_NAME: [u8; 3] = [0x55, 0x1d, 0x11];
// dNSName, [2] IMPLICIT IA5String: the one GeneralName a host is matched against.
const DNS_NAME: u8 = 0x82;

/*
 * The extension is found by walking the TBSCertificate's extension list, not by
 * searching the certificate for the OID's bytes. A search finds the first copy
 * anywhere, and the subject public key comes before the extensions: an RSA
 * modulus can be ground to carry a second, planted subjectAltName that a CA
 * then signs for whoever asked. Each GeneralName is stepped over by its own
 * DER length, long form included, so no entry's value is ever read as a name.
 */
pub fn matches(cert: &[u8], host: &[u8]) -> bool {
    let Some(value) = super::cert_ext::extension_value(cert, &OID_SUBJECT_ALT_NAME) else {
        return false;
    };
    let Some((0x30, mut pos, end)) = super::der_tlv::der_tlv(value, 0) else {
        return false;
    };
    if end != value.len() {
        return false;
    }
    while pos < end {
        let Some((tag, start, next)) = super::der_tlv::der_tlv(value, pos) else {
            return false;
        };
        if tag == DNS_NAME && host_match(&value[start..next], host) {
            return true;
        }
        pos = next;
    }
    false
}

fn host_match(name: &[u8], host: &[u8]) -> bool {
    if eq_ascii(name, host) {
        return true;
    }
    name.len() > 2 && name[0] == b'*' && name[1] == b'.' && wildcard(&name[2..], host)
}

fn wildcard(suffix: &[u8], host: &[u8]) -> bool {
    if host.len() <= suffix.len() || !eq_ascii(&host[host.len() - suffix.len()..], suffix) {
        return false;
    }
    host[host.len() - suffix.len() - 1] == b'.'
        && !host[..host.len() - suffix.len() - 1].contains(&b'.')
}

fn eq_ascii(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.eq_ignore_ascii_case(y))
}
