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

// id-ce-extKeyUsage, 2.5.29.37.
const OID_EXT_KEY_USAGE: [u8; 3] = [0x55, 0x1d, 0x25];
// id-kp-serverAuth, 1.3.6.1.5.5.7.3.1.
const SERVER_AUTH: [u8; 8] = [0x2b, 0x06, 0x01, 0x05, 0x05, 0x07, 0x03, 0x01];

/// Whether the certificate states it is for a TLS server.
///
/// Most roots let a leaf with no extKeyUsage serve anything. The throwaway
/// root's key is public, so its leaves must say so outright: absent or
/// unparseable usage denies, as does a list without serverAuth.
pub fn serves_tls(cert: &[u8]) -> bool {
    let Some(value) = crate::cert_ext::extension_value(cert, &OID_EXT_KEY_USAGE) else {
        return false;
    };
    // ExtKeyUsageSyntax ::= SEQUENCE SIZE (1..MAX) OF KeyPurposeId (an OID).
    let Some((tag, mut cur, end)) = crate::der_tlv::der_tlv(value, 0) else {
        return false;
    };
    if tag != 0x30 || end != value.len() {
        return false;
    }
    let mut server = false;
    while cur < end {
        let Some((tag, val, next)) = crate::der_tlv::der_tlv(value, cur) else {
            return false;
        };
        if tag != 0x06 || next > end {
            return false;
        }
        server |= value[val..next] == SERVER_AUTH;
        cur = next;
    }
    server
}
