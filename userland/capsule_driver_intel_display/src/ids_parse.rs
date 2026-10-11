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

//! Turns the device id data file into a table at compile time. A line
//! that does not parse, or an id listed twice, stops the build.

use crate::ids::Family;

pub struct Table {
    pub ids: [(u16, Family); 128],
    pub len: usize,
}

pub const fn parse(src: &[u8]) -> Table {
    let mut t = Table { ids: [(0, Family::Skylake); 128], len: 0 };
    let mut at = 0;
    while at < src.len() {
        if src[at] == b'#' || src[at] == b'\n' {
            while at < src.len() && src[at] != b'\n' {
                at += 1;
            }
            at += 1;
            continue;
        }
        let mut id: u32 = 0;
        while src[at] != b' ' {
            let d = match src[at] {
                c @ b'0'..=b'9' => c - b'0',
                c @ b'A'..=b'F' => c - b'A' + 10,
                _ => panic!("hex id expected"),
            };
            id = id * 16 + d as u32;
            at += 1;
        }
        assert!(id <= 0xFFFF && at + 4 <= src.len() && src[at + 4] == b'\n', "bad id line");
        let family = match [src[at + 1], src[at + 2], src[at + 3]] {
            [b's', b'k', b'l'] => Family::Skylake,
            [b'k', b'b', b'l'] => Family::KabyLake,
            [b'c', b'f', b'l'] => Family::CoffeeLake,
            [b'c', b'm', b'l'] => Family::CometLake,
            _ => panic!("family must be skl, kbl, cfl or cml"),
        };
        let mut i = 0;
        while i < t.len {
            assert!(t.ids[i].0 != id as u16, "id listed twice");
            i += 1;
        }
        assert!(t.len < 128, "table full");
        t.ids[t.len] = (id as u16, family);
        t.len += 1;
        at += 5;
    }
    t
}
