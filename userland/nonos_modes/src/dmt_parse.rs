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

//! Turns the DMT data file into a table at compile time. A malformed line
//! stops the build: a shipping table is never half read.

use crate::dmt::{Dmt, Kind};
use crate::lex::{letter, number, skip_line, spaces};
use crate::make::{timing, Axis, EMPTY};

/// Room for every mode the standard lists, and how many the file gave.
pub struct Table {
    pub modes: [Dmt; 128],
    pub len: usize,
}

pub const fn parse(src: &[u8]) -> Table {
    let mut t =
        Table { modes: [Dmt { id: 0, hz: 0, kind: Kind::Own, timing: EMPTY }; 128], len: 0 };
    let mut at = 0;
    while at < src.len() {
        if src[at] == b'#' || src[at] == b'\n' {
            at = skip_line(src, at);
            continue;
        }
        let mut f = [0u32; 10];
        let (id, mut a) = number(src, at, 16);
        let mut i = 0;
        while i < 10 {
            (f[i], a) = number(src, a, 10);
            i += 1;
        }
        let (hp, a) = letter(src, a);
        let (vp, a) = letter(src, spaces(src, a));
        let (k, a) = letter(src, a);
        let kind = match k {
            b'd' => Kind::Own,
            b'c' => Kind::Cvt,
            b'r' => Kind::Reduced,
            b'2' => Kind::ReducedV2,
            _ => panic!("kind must be d, c, r or 2"),
        };
        assert!(a == src.len() || src[a] == b'\n', "trailing text");
        assert!(t.len < 128 && id < 256 && f[0] > 0 && f[1] > 0, "bad DMT line");
        let h = Axis::new(f[0], f[4], f[5], f[6], hp == b'p');
        let v = Axis::new(f[1], f[7], f[8], f[9], vp == b'p');
        let timing = timing(f[3], h, v);
        t.modes[t.len] = Dmt { id: id as u8, hz: f[2], kind, timing };
        t.len += 1;
        at = skip_line(src, a);
    }
    t
}
