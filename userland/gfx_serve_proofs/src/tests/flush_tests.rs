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

//! FLUSH and CURSOR on the current primary.

use crate::model::wire::{ask, u32s, COMPOSITOR};
use crate::model::{Pool, Vram};
use crate::server::Server;
use nonos_gfx_wire::errno::{EINVAL, ENOSYS, ESTALE};
use nonos_gfx_wire::{ops, Cursor, Surface};

fn painted() -> (Server<Vram, Pool>, Surface) {
    let mut s = Server::new(Vram::new(true, true), Pool::new(4096));
    let (_, b) = ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]);
    let sf = Surface::decode(&b).unwrap();
    let base = s.current.unwrap().region.base;
    let m = s.platform.memory.get_mut(&base).unwrap();
    for (i, px) in m.chunks_exact_mut(4).enumerate().take(1024 * 768) {
        px.copy_from_slice(&(i as u32).to_le_bytes());
    }
    (s, sf)
}

#[test]
fn a_flush_copies_exactly_its_rectangles_into_vram() {
    let (mut s, sf) = painted();
    let (st, _) = ask(&mut s, COMPOSITOR, ops::FLUSH, 0, &u32s(&[sf.serial, 1, 10, 20, 3, 2]));
    assert_eq!(st, 0);
    let fb = &s.display.fb;
    assert_eq!(fb[20 * 1024 + 10], 20 * 1024 + 10);
    assert_eq!(fb[21 * 1024 + 12], 21 * 1024 + 12);
    assert_eq!((fb[20 * 1024 + 13], fb[22 * 1024 + 10], fb[20 * 1024 + 9]), (0, 0, 0));
}

#[test]
fn a_stale_serial_or_an_outside_rectangle_is_refused_untouched() {
    let (mut s, sf) = painted();
    assert_eq!(
        ask(&mut s, COMPOSITOR, ops::FLUSH, 0, &u32s(&[sf.serial + 1, 1, 0, 0, 1, 1])).0,
        ESTALE
    );
    assert_eq!(
        ask(&mut s, COMPOSITOR, ops::FLUSH, 0, &u32s(&[sf.serial, 1, 1020, 0, 5, 1])).0,
        EINVAL
    );
    assert!(s.display.fb.iter().all(|&p| p == 0));
}

#[test]
fn the_cursor_comes_from_the_slot_and_only_with_a_hardware_cursor() {
    let (mut s, _) = painted();
    let c = Cursor { flags: 3, x: 5, y: 6, hot_x: 1, hot_y: 1, width: 32, height: 32 };
    assert_eq!(ask(&mut s, COMPOSITOR, ops::CURSOR, 0, &c.encode()).0, 0);
    let (got, slot) = s.display.cursor.clone().unwrap();
    assert_eq!((got, slot.len()), (c, 64 * 64 * 4));
    let mut n = Server::new(Vram::new(true, false), Pool::new(4096));
    ask(&mut n, COMPOSITOR, ops::SURFACE, 0, &[]);
    assert_eq!(ask(&mut n, COMPOSITOR, ops::CURSOR, 0, &c.encode()).0, ENOSYS);
}
