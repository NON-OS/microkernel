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


//! The ClientHello offers exactly the narrow TLS 1.2 the module accepts.

use super::cases::{get, handshake};
use super::replay::SNI;

#[test]
fn the_hello_offers_tls12_two_aead_suites_p256_and_nothing_to_resume() {
    let (kind, body) = handshake(get("chacha_certreq").client).remove(0);
    assert_eq!(kind, 1);
    assert_eq!(&body[..2], &[3, 3]);
    assert_eq!(body[34], 0, "no session id: nothing to resume");
    assert_eq!(&body[35..41], &[0, 4, 0xCC, 0xA8, 0xC0, 0x30], "ChaCha20-Poly1305 and AES-256-GCM, ECDHE-RSA only");
    assert_eq!(&body[41..43], &[1, 0], "null compression only");
    let mut exts = std::vec::Vec::new();
    let mut at = 45;
    while at < body.len() {
        let kind = u16::from_be_bytes([body[at], body[at + 1]]);
        let len = usize::from(u16::from_be_bytes([body[at + 2], body[at + 3]]));
        exts.push((kind, body[at + 4..at + 4 + len].to_vec()));
        at += 4 + len;
    }
    let kinds: std::vec::Vec<u16> = exts.iter().map(|e| e.0).collect();
    assert_eq!(kinds, [0, 11, 10, 13, 23, 0xff01], "SNI, point formats, groups, signatures, EMS, renegotiation_info");
    assert!(exts[0].1.ends_with(SNI));
    assert_eq!(exts[2].1, [0, 2, 0, 23], "P-256 alone");
    assert_eq!(exts[3].1, [0, 8, 8, 4, 8, 5, 4, 1, 5, 1], "RSA-PSS and PKCS#1, SHA-256 and SHA-384");
    assert_eq!(exts[5].1, [0], "an empty renegotiated_connection: no renegotiation");
    assert!(!kinds.contains(&35) && !kinds.contains(&43), "no session ticket, no supported_versions");
}

#[test]
fn only_protocol_version_and_handshake_failure_fall_back() {
    for d in 0..=255u8 {
        assert_eq!(crate::fallback::refuses_tls13(d), d == 40 || d == 70, "alert {d}");
    }
}
