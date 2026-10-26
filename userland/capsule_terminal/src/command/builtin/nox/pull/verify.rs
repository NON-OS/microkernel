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

use alloc::vec::Vec;

pub fn sidecar_path(path: &[u8]) -> Vec<u8> {
    let mut p = Vec::with_capacity(path.len() + 7);
    p.extend_from_slice(path);
    p.extend_from_slice(b".sha256");
    p
}

pub fn parse_sha256_hex(sidecar: &[u8]) -> Option<[u8; 32]> {
    let mut it = sidecar.iter().copied().skip_while(|c| matches!(c, b' ' | b'\t' | b'\r' | b'\n'));
    let mut out = [0u8; 32];
    for b in out.iter_mut() {
        *b = (nibble(it.next()?)? << 4) | nibble(it.next()?)?;
    }
    Some(out)
}

fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

/// The verdict on a body whose hash is `got`, against the `.sha256` sidecar
/// `raw` the server keeps beside it.
pub fn judge(raw: &[u8], got: Option<[u8; 32]>) -> Result<(), &'static str> {
    let expected = parse_sha256_hex(raw).ok_or("verify: bad .sha256 sidecar")?;
    let got = got.ok_or("verify: hash failed")?;
    if got == expected {
        Ok(())
    } else {
        Err("verify: checksum mismatch")
    }
}
