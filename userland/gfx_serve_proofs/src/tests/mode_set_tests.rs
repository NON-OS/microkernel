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

//! MODE_SET makes before it breaks, and RETIRE frees the old primary.

use crate::model::wire::{ask, u32s, COMPOSITOR};
use crate::model::{Pool, Vram};
use crate::server::Server;
use nonos_gfx_wire::errno::{EBUSY, EINVAL, ENOMEM, ESTALE};
use nonos_gfx_wire::{ops, Surface};

fn set(s: &mut Server<Vram, Pool>, w: u32, h: u32) -> (i32, Vec<u8>) {
    ask(s, COMPOSITOR, ops::MODE_SET, 0, &u32s(&[w, h, 0]))
}

#[test]
fn the_new_primary_is_shown_and_answered_while_the_old_waits() {
    let mut s = Server::new(Vram::new(true, false), Pool::new(4096));
    let (_, first) = ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]);
    let old = Surface::decode(&first).unwrap();
    let (st, body) = set(&mut s, 1600, 900);
    let new = Surface::decode(&body).unwrap();
    assert_eq!((st, new.width, new.height, new.serial), (0, 1600, 900, 0x0042_0002));
    assert_eq!(s.display.size, (1600, 900));
    assert_eq!(s.retiring.unwrap().serial, old.serial, "the old one kept");
    assert_eq!(set(&mut s, 1280, 720).0, EBUSY, "one waiting at a time");
    assert_eq!(ask(&mut s, COMPOSITOR, ops::RETIRE, 0, &u32s(&[new.serial])).0, EINVAL);
    assert_eq!(ask(&mut s, COMPOSITOR, ops::RETIRE, 0, &u32s(&[old.serial])).0, 0);
    assert_eq!(s.platform.released, [old.handle]);
    assert_eq!(ask(&mut s, COMPOSITOR, ops::RETIRE, 0, &u32s(&[old.serial])).0, ESTALE);
}

#[test]
fn sizes_outside_the_range_or_list_are_refused_with_nothing_made() {
    let mut s = Server::new(Vram::new(true, false), Pool::new(4096));
    ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]);
    for (w, h) in [(639, 480), (1601, 900), (2568, 1600), (1024, 1601)] {
        assert_eq!(set(&mut s, w, h).0, EINVAL, "{w}x{h}");
    }
    let mut l = Server::new(Vram::new(false, false), Pool::new(4096));
    ask(&mut l, COMPOSITOR, ops::SURFACE, 0, &[]);
    assert_eq!(set(&mut l, 1280, 720).0, 0, "a listed mode");
    let mut l = Server::new(Vram::new(false, false), Pool::new(4096));
    assert_eq!(set(&mut l, 1600, 900).0, EINVAL, "not listed");
    assert_eq!(s.platform.memory.len(), 1);
}

#[test]
fn a_refused_or_failed_mode_set_keeps_the_old_mode_and_frees_its_attempt() {
    let mut s = Server::new(Vram::new(true, false), Pool::new(4096));
    ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]);
    s.display.fail_set = true;
    assert_eq!(set(&mut s, 1280, 720).0, EINVAL);
    assert_eq!((s.display.size, s.platform.memory.len()), ((1024, 768), 1));
    s.display.fail_set = false;
    s.platform.pages_left = 100;
    assert_eq!(set(&mut s, 2560, 1600).0, ENOMEM);
    assert!(s.retiring.is_none() && s.current.unwrap().width == 1024);
}
