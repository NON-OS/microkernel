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

//! F-105: a seeded table with one lie in it, after some honest structures:
//! a length under the header, a structure cut off by the table's end,
//! strings that never end, an early end of table, or none at all. With
//! it, what the walk must give back.

use super::seeded_gen::{encode, honest, Honest, Rng};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lie {
    Short,
    PastTable,
    Unending,
    EarlyEnd,
    None,
}

const LIES: [Lie; 5] = [Lie::Short, Lie::PastTable, Lie::Unending, Lie::EarlyEnd, Lie::None];

/// The table, the structures the walk must find, and the lie.
pub fn table(r: &mut Rng) -> (Vec<u8>, Vec<Honest>, Lie) {
    let mut found: Vec<Honest> = (0..r.below(12)).map(|_| honest(r)).collect();
    let mut t: Vec<u8> = found.iter().flat_map(encode).collect();
    let after: Vec<Honest> = (0..1 + r.below(3)).map(|_| honest(r)).collect();
    let tail: Vec<u8> = after.iter().flat_map(encode).collect();
    let lie = LIES[r.below(5) as usize];
    match lie {
        Lie::Short => t.extend([&[r.below(127) as u8, r.below(4) as u8, 0, 0], &tail[..]].concat()),
        Lie::PastTable => {
            let liar = encode(&honest(r));
            t.extend(&liar[..1 + r.below(liar.len() as u64 - 1) as usize]);
        }
        Lie::Unending => t.extend([&[r.below(127) as u8, 4, 1, 0][..], &r.some(1, 20, 1)].concat()),
        Lie::EarlyEnd => {
            t.extend([&[127u8, 4, 0, 0, 0, 0][..], &tail].concat());
            found.push(Honest { kind: 127, handle: 0, fields: vec![], strings: vec![] });
        }
        Lie::None => {
            t.extend(&tail);
            found.extend(after);
        }
    }
    (t, found, lie)
}
