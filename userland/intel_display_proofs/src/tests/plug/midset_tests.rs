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

//! A pull in the middle of a mode set: the cable goes as the new mode's
//! DPLL is set, between darkening the port and training it. The mode set
//! fails with its new surfaces given back and, the old mode not lit again
//! with the cable gone, the output waiting to be set; the next
//! settled poll leaves it dark, and the monitor back is lit at a mode set.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::body::state::SCANNING_OUT;

use super::plug_rig::{settle, watched, with_shadow};
use crate::display::Plug;
use crate::engine::aux_regs::Ddi;
use crate::engine::dpll::DPLL_CTRL1;
use crate::engine::pipe::Pipe;
use crate::tests::wire_rig::h0;

#[test]
fn a_pull_mid_mode_set_ends_dark_and_the_monitor_back_is_lit() {
    let (mut d, model, sink) = watched();
    model.borrow_mut().cable.pull_on = Some(DPLL_CTRL1);
    with_shadow(1024, 768, |s| assert!(d.set_mode(0, 1024, 768, s).is_err()));
    assert!(model.borrow().cable.pulled.is_some(), "pulled during the mode set");
    assert_eq!(h0(&d).frames.was, None, "the new surfaces given back");
    assert_eq!((h0(&d).scan.width, h0(&d).plug), (1920, Plug::Back), "the old mode not lit again");
    settle(&mut d);
    assert_eq!(h0(&d).plug, Plug::Out);
    assert_eq!(d.state(0).unwrap().flags, 0, "dark, not lost");
    model.borrow_mut().plug_in(Ddi::B, sink);
    settle(&mut d);
    assert_eq!(h0(&d).plug, Plug::Back);
    with_shadow(1920, 1080, |s| assert_eq!(d.set_mode(0, 1920, 1080, s), Ok(())));
    assert_eq!(d.state(0).unwrap().flags, SCANNING_OUT);
    assert_eq!(model.borrow_mut().pixel(Pipe::B, 1919, 1079), Some(0x7777_7777));
}
