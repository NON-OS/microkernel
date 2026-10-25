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

//! SURFACE and the owner rule.

use crate::model::wire::{ask, COMPOSITOR};
use crate::model::{Pool, Vram};
use crate::server::Server;
use nonos_gfx_wire::{ops, Surface};

#[test]
fn the_first_ask_makes_the_primary_at_the_shown_mode_and_shares_it() {
    let mut s = Server::new(Vram::new(true, true), Pool::new(4096));
    let (st, body) = ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]);
    assert_eq!(st, 0);
    let sf = Surface::decode(&body).unwrap();
    assert_eq!((sf.width, sf.height, sf.stride, sf.serial), (1024, 768, 4096, 0x0042_0001));
    assert_eq!(sf.cursor_offset, 4096 * 768, "the cursor slot after the frame");
    assert_eq!(s.platform.shares, [(sf.handle, COMPOSITOR)]);
    let (_, again) = ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]);
    assert_eq!(Surface::decode(&again).unwrap(), sf, "the same primary");
}

#[test]
fn a_restarted_compositor_gets_the_primary_and_the_retiring_one_is_freed() {
    let mut s = Server::new(Vram::new(true, false), Pool::new(4096));
    ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]);
    let (st, _) =
        ask(&mut s, COMPOSITOR, ops::MODE_SET, 0, &crate::model::wire::u32s(&[1280, 720, 0]));
    assert_eq!(st, 0);
    let old = s.slots[0].retiring.unwrap().handle;
    let (st, body) = ask(&mut s, COMPOSITOR + 1, ops::SURFACE, 0, &[]);
    assert_eq!(st, 0);
    assert_eq!(Surface::decode(&body).unwrap().width, 1280);
    assert!(s.slots[0].retiring.is_none() && s.platform.released == [old]);
    assert_eq!(s.owner, Some(COMPOSITOR + 1));
}

#[test]
fn a_full_pool_is_enomem_and_nothing_leaks() {
    let mut s = Server::new(Vram::new(true, false), Pool::new(100));
    let (st, _) = ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]);
    assert_eq!(st, nonos_gfx_wire::errno::ENOMEM);
    assert!(s.slots[0].current.is_none() && s.platform.memory.is_empty());
}
