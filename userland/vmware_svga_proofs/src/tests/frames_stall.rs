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

//! A host that stops reading costs one refused flush, not a hang, and the
//! next flush once it reads again is redrawn.

use super::ask::{ask, primed, words};
use super::util::up;
use crate::bench::Bench;
use crate::model::with_svga;
use nonos_gfx_wire::errno::ETIMEDOUT;
use nonos_gfx_wire::ops;

#[test]
fn a_host_that_stops_reading_drops_the_update_after_a_bounded_wait() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    let sf = primed(&mut d.server, 1);
    with_svga(|s| s.stalled = true);
    let flush = words(&[sf.serial, 1, 0, 0, 8, 8]);
    let refused = (0..4000).find(|_| ask(&mut d.server, ops::FLUSH, &flush).0 == ETIMEDOUT);
    assert!(refused.is_some(), "a full ring answers ETIMEDOUT");
    with_svga(|s| s.stalled = false);
    assert_eq!(ask(&mut d.server, ops::FLUSH, &flush).0, 0, "and recovers once the host reads");
}
