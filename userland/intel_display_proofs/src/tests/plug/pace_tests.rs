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

//! The serve loop's tick polls only once `POLL_US` have passed, so the
//! two polls a change needs to settle are a poll interval apart however
//! many frames come in between.

use super::plug_rig::watched;
use crate::display::{Plug, POLL_US};
use crate::tests::wire_rig::h0;

#[test]
fn a_pull_settles_after_two_poll_intervals_not_two_frames() {
    let (mut d, model, _) = watched();
    // One poll first, so the interval starts here.
    d.clock.now.set(d.clock.now.get() + POLL_US);
    assert!(!d.tick(), "the line high, as it was");
    model.borrow_mut().pull();
    let start = d.clock.now.get();
    while d.clock.now.get() < start + POLL_US - 1000 {
        assert!(!d.tick());
    }
    assert_eq!(h0(&d).plug, Plug::In, "frames alone poll nothing");
    let told = (0..2).map(|_| {
        d.clock.now.set(d.clock.now.get() + POLL_US);
        d.tick()
    });
    assert_eq!(told.collect::<Vec<_>>(), [false, true], "told once, as it settles");
    assert_eq!(h0(&d).plug, Plug::Out);
}
