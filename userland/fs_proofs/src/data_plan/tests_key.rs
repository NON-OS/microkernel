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

//! The key header: read back as written, refused by name when unknown, and
//! the volume key it seals opened only by the passphrase that sealed it.

use super::key_header::{parse_key_header, Keyed, Sealed};
use super::key_seal::{encode_key_header, seal_key, unseal_key};
use crate::crypto::argon2::{argon2id, Params};

const SMALL: Params = Params { m_kib: 64, t: 1, p: 1 };

fn kek(passphrase: &[u8], salt: &[u8; 32]) -> [u8; 32] {
    let mut out = [0u8; 32];
    argon2id(passphrase, salt, SMALL, &mut out, &mut || {}).unwrap();
    out
}

fn sealed_by(passphrase: &[u8], key: &[u8; 32]) -> Sealed {
    let mut s = Sealed { params: SMALL, salt: [7; 32], nonce: [9; 12], key_and_tag: [0; 48] };
    assert!(seal_key(&kek(passphrase, &s.salt), &mut s, key));
    s
}

#[test]
fn headers_read_back_as_written_and_a_blank_sector_is_none() {
    let s = sealed_by(b"correct horse", &[5; 32]);
    for keyed in [Keyed::Tpm, Keyed::Passphrase(s)] {
        assert_eq!(parse_key_header(&encode_key_header(&keyed)), Ok(Some(keyed)));
    }
    assert_eq!(parse_key_header(&[0u8; 512]), Ok(None));
    let mut unknown = encode_key_header(&Keyed::Tpm);
    unknown[8] = 9;
    assert_eq!(parse_key_header(&unknown), Err(9));
}

#[test]
fn only_the_sealing_passphrase_opens_the_volume_key() {
    let s = sealed_by(b"correct horse", &[5; 32]);
    assert_eq!(unseal_key(&kek(b"correct horse", &s.salt), &s), Some([5; 32]));
    assert_eq!(unseal_key(&kek(b"correct horsf", &s.salt), &s), None);
    assert_eq!(unseal_key(&kek(b"", &s.salt), &s), None);
}

#[test]
fn a_changed_parameter_salt_or_sealed_byte_fails_the_tag() {
    let s = sealed_by(b"pw pw pw pw", &[1; 32]);
    let right = kek(b"pw pw pw pw", &s.salt);
    for at in [12usize, 16, 20, 24, 55, 56, 67, 68, 115] {
        let mut sector = encode_key_header(&Keyed::Passphrase(s));
        sector[at] ^= 1;
        let Ok(Some(Keyed::Passphrase(bent))) = parse_key_header(&sector) else { panic!() };
        assert_eq!(unseal_key(&right, &bent), None, "byte {at}");
    }
}
