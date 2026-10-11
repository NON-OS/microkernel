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

//! The eDP panel power sequencer.

use crate::engine::error::Error;
use crate::engine::pps::{backlight, off, on, BACKLIGHT, KEY, ON, PP_CONTROL, PP_STATUS};
use crate::model::{Gen9, Polls};

#[test]
fn power_on_waits_for_the_sequence_to_finish_lit() {
    let mut m = Gen9::new(1);
    on(&mut m, &Polls::new(30)).unwrap();
    assert_eq!(m.get(PP_STATUS) & ON, ON);
    assert_eq!(m.get(PP_CONTROL) & 0xFFFF_0000, KEY, "the write carried its key");
    m.writes.clear();
    on(&mut m, &Polls::new(30)).unwrap();
    assert!(m.writes.is_empty(), "an already lit panel is left alone");
}

#[test]
fn power_off_takes_the_backlight_down_with_the_panel() {
    let mut m = Gen9::new(1);
    on(&mut m, &Polls::new(30)).unwrap();
    backlight(&mut m, true);
    assert_eq!(m.get(PP_CONTROL) & BACKLIGHT, BACKLIGHT);
    off(&mut m, &Polls::new(30)).unwrap();
    assert_eq!((m.get(PP_STATUS) & ON, m.get(PP_CONTROL) & BACKLIGHT), (0, 0));
}

#[test]
fn a_sequencer_that_never_finishes_times_out() {
    let mut m = Gen9::new(1);
    m.dead = true;
    assert_eq!(on(&mut m, &Polls::new(30)), Err(Error::Timeout));
}
