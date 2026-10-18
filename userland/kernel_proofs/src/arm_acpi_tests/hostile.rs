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

//! Every table cut short at every length, and every byte set to values
//! that break lengths, counts and references, resealed so the header
//! checks pass and the parser meets the damage itself. Nothing may panic,
//! loop or index out of bounds; what it answers is its own business.

use super::build::seal;

pub fn every_cut_and_byte<T, E>(t: &[u8], parse: impl Fn(&[u8]) -> Result<T, E>) {
    assert!(parse(t).is_ok(), "the table as built parses");
    let mut refused = 0;
    for len in 0..t.len() {
        let mut cut = t[..len].to_vec();
        if cut.len() >= 36 {
            seal(&mut cut);
        }
        let _ = parse(&cut);
        let _ = parse(&t[..len]);
    }
    // From the revision on; seal rewrites the length and checksum.
    for at in 8..t.len() {
        for v in [0u8, 1, 2, 3, 4, 0x10, 0x14, 0x24, 0x7F, 0x80, 0xFE, 0xFF] {
            let mut m = t.to_vec();
            m[at] = v;
            seal(&mut m);
            refused += usize::from(parse(&m).is_err());
        }
    }
    assert!(refused > 0, "some damage is refused, so the sweep reached the parser");
}
