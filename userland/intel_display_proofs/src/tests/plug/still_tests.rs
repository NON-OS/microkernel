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

//! What never darkens an output showing a picture: a line low from the
//! start (firmware never armed it, or the board does not wire it), and a
//! bounce shorter than two polls. Only a pull two polls agree on does.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};

use super::plug_rig::{settle, watched};
use crate::display::Plug;
use crate::engine::aux_regs::Ddi;
use crate::model::firmware::T1080;
use crate::tests::dp_rig::dp_rig;
use crate::tests::edid_blob::edid_1080p_and_xga;
use crate::tests::two_rig::TOTAL;
use crate::tests::wire_rig::h0;

#[test]
fn a_line_low_from_the_start_never_darkens_a_lit_output() {
    let (mut d, model, _) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), TOTAL);
    d.watch();
    let _ = d.state(0);
    let before = model.borrow().writes.len();
    settle(&mut d);
    settle(&mut d);
    assert_eq!(h0(&d).plug, Plug::In);
    assert_eq!(model.borrow().writes.len(), before, "nothing darkened");
    assert_eq!(d.state(0).unwrap().flags, SCANNING_OUT);
}

#[test]
fn a_bounce_shorter_than_two_polls_leaves_the_output_lit() {
    let (mut d, model, sink) = watched();
    let before = model.borrow().writes.len();
    model.borrow_mut().pull();
    d.poll_plugs();
    model.borrow_mut().plug_in(Ddi::B, sink);
    settle(&mut d);
    assert_eq!(h0(&d).plug, Plug::In);
    assert_eq!(model.borrow().writes.len(), before, "nothing darkened");
    let flags = d.state(0).unwrap().flags;
    assert_eq!(flags, SCANNING_OUT | LOST, "the link the bounce dropped trained again");
    assert_eq!(d.state(0).unwrap().flags, SCANNING_OUT);
}
