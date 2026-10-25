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

use super::dmar_walk::FIRST_STRUCTURE;

pub(super) struct Seed(pub(super) u64);

impl Seed {
    fn below(&mut self, n: u64) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0 % n
    }
}

/// A DMAR of random structures, RMRRs with whole-page regions, one structure
/// in eight lying about its length, one table in ten cut short anywhere.
pub(super) fn table(s: &mut Seed) -> Vec<u8> {
    let mut t = vec![0u8; FIRST_STRUCTURE];
    for _ in 0..s.below(12) {
        let kind = [0u16, 1, 1, 2, 4, 9, 0xFFFF][s.below(7) as usize];
        let body = s.below(48) as usize + 24;
        let length = [s.below(4), s.below(0x1_0000)].get(s.below(16) as usize).copied();
        let base = s.below(1 << 20) << 12;
        let limit = (base + (s.below(16) << 12)) | 0xFFF;
        let start = t.len();
        t.extend(
            kind.to_le_bytes().iter().chain(&(length.unwrap_or(body as u64) as u16).to_le_bytes()),
        );
        t.extend([0u8; 4].iter().chain(&base.to_le_bytes()).chain(&limit.to_le_bytes()));
        (start + 24..start + body).for_each(|_| t.push(s.below(256) as u8));
    }
    if s.below(10) == 0 {
        t.truncate(s.below(t.len() as u64 + 1) as usize);
    }
    t
}
