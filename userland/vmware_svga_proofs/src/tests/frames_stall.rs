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

//! A host that stops reading costs one dropped update, not a hang.

use super::frames::body;
use super::util::up;
use crate::bench::Bench;
use crate::model::with_svga;
use crate::server::handlers::flush;

#[test]
fn a_host_that_stops_reading_drops_the_update_after_a_bounded_wait() {
    let mut bench = Bench::standard();
    let mut d = up(&mut bench, |_| {});
    with_svga(|s| s.stalled = true);
    let mut refused = 0;
    for _ in 0..4000 {
        if flush(&mut d, &body(&[1, 0, 0, 8, 8], None)) == -110 {
            refused += 1;
            break;
        }
    }
    assert_eq!(refused, 1, "a full ring answers E_DEVICE");
    with_svga(|s| s.stalled = false);
    assert_eq!(flush(&mut d, &body(&[1, 0, 0, 8, 8], None)), 0, "and recovers once the host reads");
}
