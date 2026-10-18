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

//! Several outputs on one server: each its own primaries, serials unique
//! across them, one owner, and no output's request touching another's.

use crate::model::wire::{ask, u32s, COMPOSITOR};
use crate::model::{Heads, Pool};
use crate::server::Server;
use nonos_gfx_wire::errno::{ENXIO, ESTALE};
use nonos_gfx_wire::{ops, State, Surface};

pub(super) type Two = Server<Heads, Pool>;

pub(super) fn two() -> Two {
    Server::new(Heads::new(&[(1920, 1080), (1280, 1024)]), Pool::new(8192))
}

pub(super) fn surface(s: &mut Two, o: u16, from: u32) -> Surface {
    let (st, body) = ask(s, from, ops::SURFACE, o, &[]);
    assert_eq!(st, 0);
    Surface::decode(&body).unwrap()
}

fn flush(s: &mut Two, o: u16, serial: u32, w: u32, h: u32) -> i32 {
    ask(s, COMPOSITOR, ops::FLUSH, o, &u32s(&[serial, 1, 0, 0, w, h])).0
}

#[test]
fn each_output_has_its_own_primary_at_its_own_mode() {
    let mut s = two();
    let (a, b) = (surface(&mut s, 0, COMPOSITOR), surface(&mut s, 1, COMPOSITOR));
    assert_eq!((a.width, a.height, b.width, b.height), (1920, 1080, 1280, 1024));
    assert_ne!(a.serial, b.serial);
    assert_ne!(a.handle, b.handle);
    let st = |s: &mut Two, o| State::decode(&ask(s, COMPOSITOR, ops::STATE, o, &[]).1).unwrap();
    assert_eq!((st(&mut s, 0).serial, st(&mut s, 1).serial), (a.serial, b.serial));
    let base = s.slots[1].current.unwrap().region.base;
    s.platform.memory.get_mut(&base).unwrap()[..4].copy_from_slice(&[1, 2, 3, 4]);
    assert_eq!(flush(&mut s, 1, a.serial, 1, 1), ESTALE, "output 0's serial on output 1");
    assert_eq!(flush(&mut s, 1, b.serial, 1, 1), 0);
    assert_eq!((s.display.heads[1].fb[0], s.display.heads[0].fb[0]), (0x0403_0201, 0));
}

#[test]
fn outputs_past_the_device_or_the_server_are_refused() {
    let mut s = two();
    assert_eq!(ask(&mut s, COMPOSITOR, ops::SURFACE, 2, &[]).0, ENXIO);
    let six: Vec<_> = (0..6).map(|i| (640 + 8 * i, 480)).collect();
    let mut s = Server::new(Heads::new(&six), Pool::new(8192));
    assert_eq!(ask(&mut s, COMPOSITOR, ops::SURFACE, 3, &[]).0, 0);
    assert_eq!(ask(&mut s, COMPOSITOR, ops::STATE, 4, &[]).0, ENXIO, "past MAX_OUTPUTS");
}
