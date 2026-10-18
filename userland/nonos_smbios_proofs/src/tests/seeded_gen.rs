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

//! F-105: seeded SMBIOS structures, by F-98's method: honest ones of random
//! kinds, fields and string sets, kept as the model the walk must give back.

pub struct Rng(pub u64);

impl Rng {
    pub fn below(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n.max(1)
    }
    /// `min` to `most` bytes, each at least `lo`.
    pub fn some(&mut self, min: u64, most: u64, lo: u8) -> Vec<u8> {
        let n = min + self.below(most - min + 1);
        self.bytes(n, lo)
    }
    pub fn bytes(&mut self, n: u64, lo: u8) -> Vec<u8> {
        (0..n).map(|_| lo + (self.below(u64::from(255 - lo)) as u8)).collect()
    }
}

/// One structure as the model knows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Honest {
    pub kind: u8,
    pub handle: u16,
    pub fields: Vec<u8>,
    pub strings: Vec<Vec<u8>>,
}

pub fn honest(r: &mut Rng) -> Honest {
    let strings = (0..r.below(5)).map(|_| r.some(1, 12, 1)).collect();
    let (kind, handle) = (r.below(127) as u8, r.below(0x10000) as u16);
    Honest { kind, handle, fields: r.some(0, 39, 0), strings }
}

pub fn encode(h: &Honest) -> Vec<u8> {
    let mut v = vec![h.kind, (4 + h.fields.len()) as u8];
    v.extend(h.handle.to_le_bytes());
    v.extend(&h.fields);
    for s in &h.strings {
        v.extend(s);
        v.push(0);
    }
    v.extend(if h.strings.is_empty() { &[0u8, 0][..] } else { &[0][..] });
    v
}
