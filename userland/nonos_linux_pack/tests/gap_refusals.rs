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

//! One pack has one form: a byte that is no file's, set anywhere between
//! the names and the end, refuses the whole pack, though every file in it
//! still reads as its digest.

mod common;

use common::vimish;
use nonos_linux_pack::{build, Pack};

#[test]
fn a_byte_in_a_gap_refuses_the_pack() {
    let bytes = build(vimish()).expect("built");
    let p = Pack::parse(&bytes).expect("parsed");
    let ends = |i| p.entry(i).map(|e| (e.name.as_ptr() as usize - bytes.as_ptr() as usize, e));
    let names_end = (0..p.len()).filter_map(|i| ends(i).ok()).map(|(at, e)| at + e.name.len());
    let from = names_end.max().expect("a name");
    let owned = |at: usize| (0..p.len()).any(|i| p.entry(i).is_ok_and(|e| e.data.contains(&at)));
    let gaps: Vec<usize> = (from..bytes.len()).filter(|&at| !owned(at)).collect();
    assert!(gaps.iter().any(|&at| at > 4096), "a gap between two files");
    for at in gaps {
        let mut bad = bytes.clone();
        bad[at] = 0xa5;
        assert!(Pack::parse(&bad).is_err(), "a byte at {at} that is no file's");
    }
}
