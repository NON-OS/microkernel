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

//! The rest of the ops, and frames that are not right.

use crate::model::wire::{ask, COMPOSITOR};
use crate::model::{Pool, Vram};
use crate::server::Server;
use nonos_gfx_wire::errno::{EINVAL, ENOSYS, ENXIO};
use nonos_gfx_wire::{ops, Caps, Modes, State};

#[test]
fn caps_modes_and_state_answer_from_the_device() {
    let mut s = Server::new(Vram::new(false, false), Pool::new(4096));
    let (_, c) = ask(&mut s, COMPOSITOR, ops::CAPS, 0, &[]);
    assert_eq!(Caps::decode(&c).unwrap().max_width, 2560);
    let (_, m) = ask(&mut s, COMPOSITOR, ops::MODES, 0, &[]);
    assert_eq!(Modes::decode(&m).unwrap().len, 2);
    ask(&mut s, COMPOSITOR, ops::SURFACE, 0, &[]);
    let (_, st) = ask(&mut s, COMPOSITOR, ops::STATE, 0, &[]);
    assert_eq!(State::decode(&st).unwrap().serial, 0x0042_0001, "the server fills the serial");
    assert_eq!(ask(&mut s, COMPOSITOR, ops::HEALTHCHECK, 0, &[]).0, 0);
}

#[test]
fn another_output_an_unknown_op_or_a_short_body_is_refused() {
    let mut s = Server::new(Vram::new(true, false), Pool::new(4096));
    assert_eq!(ask(&mut s, COMPOSITOR, ops::CAPS, 1, &[]).0, ENXIO);
    assert_eq!(ask(&mut s, COMPOSITOR, 0x30, 0, &[]).0, ENOSYS);
    assert_eq!(ask(&mut s, COMPOSITOR, ops::MODE_SET, 0, &[1, 2, 3]).0, EINVAL);
    assert_eq!(ask(&mut s, COMPOSITOR, ops::FLUSH, 0, &[0; 8]).0, EINVAL);
    let mut out = [0u8; 64];
    assert_eq!(s.handle(COMPOSITOR, &[0u8; 20], &mut out), 0, "no header, no reply");
}
