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

//! Every honest body either end writes reads back as written.

use super::rng::Rng;
use crate::body::rect::Rect;
use crate::body::{caps::Caps, cursor::Cursor, flush::Flush, state::State, surface::Surface};

#[test]
fn every_honest_body_round_trips() {
    let mut r = Rng::new(0x20_7219);
    for _ in 0..20_000 {
        let w = |r: &mut Rng| r.word().max(1);
        let caps = Caps {
            outputs: w(&mut r) as u16 | 1,
            width_align: w(&mut r) as u16 | 1,
            flags: r.word(),
            max_width: r.word(),
            ..Caps::default()
        };
        assert_eq!(Caps::decode(&caps.encode()), Ok(caps));
        let width = r.below(1 << 20) + 1;
        let s = Surface {
            handle: r.next(),
            width,
            height: w(&mut r),
            stride: width * 4 + r.below(64),
            format: 1,
            cursor_offset: r.word(),
            serial: r.word(),
        };
        assert_eq!(Surface::decode(&s.encode()), Ok(s));
        let st =
            State { generation: r.word(), width: r.word(), flags: r.word(), ..State::default() };
        assert_eq!(State::decode(&st.encode()), Ok(st));
        let side = r.below(65) as u16;
        let c = Cursor {
            width: side,
            height: side,
            hot_x: r.below(side.max(1) as u32) as u16,
            x: r.word() as i32,
            ..Cursor::default()
        };
        assert_eq!(Cursor::decode(&c.encode()), Ok(c));
        let n = 1 + r.below(15) as usize;
        let mut f = Flush { serial: r.word(), len: n, ..Flush::default() };
        f.rects[..n]
            .iter_mut()
            .for_each(|x| *x = Rect { x: r.word(), y: 1, width: 2, height: r.word() });
        let mut out = [0u8; 8 + 16 * 15];
        let k = f.encode(&mut out).unwrap();
        assert_eq!(Flush::decode(&out[..k]), Ok(f));
    }
}
