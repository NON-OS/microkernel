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

//! HKDF-SHA-512 against the 83 published vectors in hkdf512_vectors.txt;
//! a request past 255 blocks is refused before anything is written.

use nonos_hash::{hkdf512_expand, hkdf512_extract, HkdfTooLong};

fn hex(s: &str) -> Vec<u8> {
    if s == "-" {
        return Vec::new();
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

fn derive(ikm: &[u8], salt: &[u8], info: &[u8], n: usize) -> Result<Vec<u8>, HkdfTooLong> {
    let mut out = vec![0xA5; n];
    let r = hkdf512_expand(&hkdf512_extract(salt, ikm), info, &mut out);
    if r.is_err() {
        assert!(out.iter().all(|b| *b == 0xA5), "nothing is written on refusal");
    }
    r.map(|_| out)
}

#[test]
fn every_published_vector() {
    let mut seen = (0, 0);
    for line in include_str!("hkdf512_vectors.txt").lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = line.split(' ').collect();
        let (id, valid, size) = (f[0], f[1] == "valid", f[2].parse::<usize>().unwrap());
        let got = derive(&hex(f[3]), &hex(f[4]), &hex(f[5]), size);
        match valid {
            true => assert_eq!(got.as_deref(), Ok(&hex(f[6])[..]), "vector {id}"),
            false => assert_eq!(got, Err(HkdfTooLong), "vector {id}"),
        }
        if valid {
            seen.0 += 1
        } else {
            seen.1 += 1
        }
    }
    assert_eq!(seen, (80, 3));
}
