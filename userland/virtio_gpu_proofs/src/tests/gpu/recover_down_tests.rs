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

//! A host that never answers again leaves the driver Down after three
//! tries. Down keeps the claim and every grant, answers LOST and dark under
//! a new generation, refuses FLUSH with ENXIO, and is reported on one turn
//! only, so the compositor is told LEAVING once.

use nonos_gfx_wire::body::state::LOST;
use nonos_gfx_wire::errno::ENXIO;
use nonos_libc::{releases, said};

use super::kit::{draw, live, state};
use super::recover_kit::{flush, said_from, stall};
use super::rig::rig;
use crate::server::recover::settle;
use crate::state::Health;

#[test]
fn a_host_that_never_answers_again_leaves_the_driver_down_holding_everything() {
    let mut r = rig();
    let s = draw(&mut r, 0, 0x0011_2233);
    let (grants, given, lines) = (live(), releases(), said().len());
    let before = state(&mut r, 0).generation;
    assert!(stall(&mut r, &s, 0), "down on this turn");
    let said = said_from(lines);
    for t in 1..=3 {
        assert!(said.iter().any(|l| l.contains(&format!("(try {t} of 3)"))), "try {t}");
    }
    assert_eq!(r.d.server.display.health, Health::Down);
    assert_eq!((live(), releases()), (grants, given), "the claim and every grant kept");
    let st = state(&mut r, 0);
    assert_eq!(st.flags, LOST, "LOST and not scanning out");
    assert_ne!(st.generation, before, "a new generation, so the compositor looks");
    assert_eq!(flush(&mut r, &s), ENXIO);
    assert!(!settle(&mut r.d), "reported once");
}
