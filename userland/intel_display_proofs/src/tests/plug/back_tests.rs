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

//! A monitor back in the port: its EDID read again (another monitor may be
//! in it), LOST until a mode set lights it on the new monitor; and one not
//! answering yet is read again on every poll.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};

use super::plug_rig::{settle, watched, with_shadow};
use crate::display::Plug;
use crate::engine::aux_regs::Ddi;
use crate::engine::pipe::Pipe;
use crate::model::sink::sink;
use crate::tests::edid_blob::edid_1080p_and_xga;
use crate::tests::wire_rig::h0;

#[test]
fn another_monitor_plugged_in_is_read_and_lit_at_the_mode_shown() {
    let (mut d, model, _) = watched();
    let was = h0(&d).panel.physical_mm;
    model.borrow_mut().pull();
    settle(&mut d);
    let other = sink(edid_1080p_and_xga((520, 290)));
    model.borrow_mut().plug_in(Ddi::B, other.clone());
    d.poll_plugs();
    assert_eq!(h0(&d).plug, Plug::Out, "not settled yet");
    d.poll_plugs();
    assert_eq!(h0(&d).plug, Plug::Back);
    assert_ne!(h0(&d).panel.physical_mm, was, "the new monitor's own EDID");
    assert_eq!(d.state(0).unwrap().flags, LOST, "lost until lit");
    assert_eq!(d.state(0).unwrap().flags, LOST);
    with_shadow(1920, 1080, |s| assert_eq!(d.set_mode(0, 1920, 1080, s), Ok(())));
    assert_eq!(h0(&d).plug, Plug::In);
    assert!(other.borrow().link.is_some(), "trained on the new monitor");
    assert_eq!(d.state(0).unwrap().flags, SCANNING_OUT);
    assert_eq!(model.borrow_mut().pixel(Pipe::B, 9, 9), Some(0x7777_7777));
}

#[test]
fn a_monitor_whose_edid_does_not_answer_yet_is_read_on_every_poll() {
    let (mut d, model, sink) = watched();
    model.borrow_mut().pull();
    settle(&mut d);
    sink.borrow_mut().nack = true;
    model.borrow_mut().plug_in(Ddi::B, sink.clone());
    settle(&mut d);
    d.poll_plugs();
    assert_eq!(h0(&d).plug, Plug::Out, "still waking");
    sink.borrow_mut().nack = false;
    d.poll_plugs();
    assert_eq!(h0(&d).plug, Plug::Back, "read on the next poll");
}
