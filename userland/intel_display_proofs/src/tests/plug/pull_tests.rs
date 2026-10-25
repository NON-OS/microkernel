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

//! A pull: once two polls agree the port is darkened, plane to DPLL, and
//! the output answers dark, touching no register for anything sent to it.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::errno::EAGAIN;
use nonos_gfx_wire::{Cursor, Rect};

use super::plug_rig::{settle, watched, with_shadow};
use crate::display::Plug;
use crate::engine::ddi::{buf_ctl, tp_ctl};
use crate::engine::dpll::enable_reg;
use crate::engine::hotplug::{ENABLES, SHOTPLUG_CTL};
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::plane_regs::plane_ctl;
use crate::engine::regs::trans_conf;
use crate::tests::wire_rig::{h0, rig};

#[test]
fn watching_arms_detection_and_needs_an_output_with_mode_setting() {
    let (d, model, _) = watched();
    assert_eq!(model.borrow().get(SHOTPLUG_CTL) & ENABLES, ENABLES);
    assert!(d.plugs.is_some());
    let (mut edp, model) = rig();
    let before = model.borrow().writes.len();
    edp.watch();
    assert!(edp.plugs.is_none(), "eDP alone: nothing watched");
    assert_eq!(model.borrow().writes.len(), before, "nor armed");
}

#[test]
fn a_pull_darkens_the_port_once_two_polls_agree() {
    let (mut d, model, _) = watched();
    model.borrow_mut().pull();
    assert!(!d.poll_plugs());
    assert_eq!(h0(&d).plug, Plug::In, "one poll is not enough");
    assert!(d.poll_plugs(), "an output moved: the compositor is told");
    assert_eq!(h0(&d).plug, Plug::Out);
    let m = model.borrow();
    assert_eq!(m.get(trans_conf(Transcoder::B)) >> 31, 0, "transcoder off");
    assert_eq!(m.get(plane_ctl(Pipe::B, 1)) >> 31, 0, "plane off");
    assert_eq!((m.get(buf_ctl(1)), m.get(tp_ctl(1))), (0, 0), "port off");
    assert_eq!(m.get(enable_reg(1)) >> 31, 0, "its DPLL off");
}

#[test]
fn a_pulled_output_answers_dark_and_writes_nothing() {
    let (mut d, model, _) = watched();
    model.borrow_mut().pull();
    settle(&mut d);
    let before = model.borrow().writes.len();
    assert_eq!(d.state(0).unwrap().flags, 0, "neither scanning out nor lost");
    let all = [Rect { x: 0, y: 0, width: 1920, height: 1080 }];
    with_shadow(1920, 1080, |s| assert_eq!(d.flush(0, &all, s), Ok(())));
    assert_eq!(d.cursor(0, &Cursor::default(), &[0; 16384]), Ok(()));
    with_shadow(1024, 768, |s| assert_eq!(d.set_mode(0, 1024, 768, s), Err(EAGAIN)));
    assert_eq!(model.borrow().writes.len(), before, "no register written");
}
