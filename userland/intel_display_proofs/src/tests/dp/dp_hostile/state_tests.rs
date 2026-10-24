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

//! A link the sink will not report on is not left as fine: STATE says the
//! output is lost, not scanning out.

use nonos_gfx_serve::Display;
use nonos_gfx_wire::body::state::{LOST, SCANNING_OUT};

use crate::model::firmware::T1080;
use crate::tests::dp_rig::dp_rig;
use crate::tests::edid_blob::edid_1080p_and_xga;
use crate::tests::two_rig::TOTAL;

#[test]
fn a_sink_that_stops_answering_is_lost_on_state() {
    let (mut d, _, sink) = dp_rig(T1080, edid_1080p_and_xga((600, 340)), TOTAL);
    let _ = d.state(0);
    assert_eq!(d.state(0).unwrap().flags, SCANNING_OUT, "the link holds");
    sink.borrow_mut().nack = true;
    assert_eq!(d.state(0).unwrap().flags & (LOST | SCANNING_OUT), LOST);
}
