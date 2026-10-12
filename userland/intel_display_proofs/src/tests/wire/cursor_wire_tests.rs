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

//! CURSOR on a taken-over output: a new shape is copied out of the
//! primary's cursor slot into the cursor's own memory, mapped once at the
//! bottom of the output's GGTT window, and the plane shown at the pointer
//! less its hot spot; a move maps nothing again; hiding keeps the image.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::body::caps::HW_CURSOR;
use nonos_gfx_wire::body::cursor::{NEW_SHAPE, VISIBLE};
use nonos_gfx_wire::Cursor;

use crate::engine::cursor::position;
use crate::engine::pipe::Pipe;
use crate::engine::plane_regs as r;
use crate::model::frames::CURSOR_IOVA;
use crate::tests::two_rig::two_rig;

fn arrow(flags: u32, x: i32, y: i32) -> Cursor {
    Cursor { flags, x, y, hot_x: 2, hot_y: 3, width: 16, height: 16 }
}

#[test]
fn a_shape_is_mapped_once_and_the_plane_follows_the_pointer() {
    let (mut d, model, windows) = two_rig();
    assert_ne!(d.caps().flags & HW_CURSOR, 0);
    let slot: Vec<u8> = (0..64 * 64 * 4).map(|i| i as u8).collect();
    d.cursor(1, &arrow(NEW_SHAPE | VISIBLE, 100, 50), &slot).unwrap();
    model.borrow_mut().vblank();
    let at = windows[1].0;
    {
        let m = model.borrow();
        assert_eq!(m.ggtt[at as usize] & !0xFFF, CURSOR_IOVA, "mapped at the window's bottom");
        assert_eq!(m.memory[&CURSOR_IOVA][..4], slot[..4], "the slot's image copied");
        assert_eq!(m.live_reg(r::cur_base(Pipe::B)), at * 4096);
        assert_eq!(m.live_reg(r::cur_pos(Pipe::B)), position(98, 47), "less the hot spot");
    }
    model.borrow_mut().writes.clear();
    d.cursor(1, &arrow(VISIBLE, 300, 200), &slot).unwrap();
    let m = model.borrow();
    assert!(m.writes.iter().all(|w| w.0 < 0x80_0000), "no GGTT entry written for a move");
}

#[test]
fn hiding_keeps_the_image_and_a_shapeless_cursor_stays_hidden() {
    let (mut d, model, _) = two_rig();
    let slot = vec![0xFFu8; 64 * 64 * 4];
    d.cursor(0, &arrow(VISIBLE, 10, 10), &slot).unwrap();
    model.borrow_mut().vblank();
    assert_eq!(model.borrow().live_reg(r::cur_ctl(Pipe::A)) & 0x3F, 0, "no shape yet: hidden");
    d.cursor(0, &arrow(NEW_SHAPE | VISIBLE, 10, 10), &slot).unwrap();
    d.cursor(0, &arrow(0, 10, 10), &slot).unwrap();
    model.borrow_mut().vblank();
    let m = model.borrow();
    assert_eq!(m.live_reg(r::cur_ctl(Pipe::A)) & 0x3F, 0, "hidden");
    assert_ne!(m.live_reg(r::cur_base(Pipe::A)), 0, "its image kept");
}
