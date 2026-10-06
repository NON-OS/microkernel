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

//! Seeded mutation fuzzing of the armor reader, not coverage-guided: every
//! damaged armored key or signature returns, in a debug build, without a
//! panic or an overflow.

use nonos_openpgp::{dearmor, keys};

#[test]
fn damaged_armor_never_panics() {
    let mut s = 0xA2_40u64;
    let mut next = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    for name in ["rsa.asc", "rsa-sha256.asc"] {
        let path = format!("{}/tests/vectors/{name}", env!("CARGO_MANIFEST_DIR"));
        let clean = std::fs::read(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        for _ in 0..4000 {
            let mut v = clean.clone();
            let at = (next() % v.len() as u64) as usize;
            match next() % 3 {
                0 => v[at] = next() as u8,
                1 => v.truncate(at),
                _ => v.insert(at, b"=\n-A"[(next() % 4) as usize]),
            }
            if let Some(bin) = dearmor(&v) {
                let _ = keys(&bin);
            }
        }
    }
}
