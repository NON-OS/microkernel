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

//! One verb over the Immediate Command interface (HDA 1.0a 3.4.3) against a
//! status register that keeps IRV until it is written as 1: a result a
//! timed-out verb left valid is cleared before the post, never read as the
//! new verb's answer.

use super::TO_0;
use crate::clock::on_poll;
use crate::constants::{ICIS, ICIS_IRV, ICOI};
use crate::controller::immediate::exchange;
use crate::model::{window, Immediate};
use crate::regs::Regs;

const STALE: u32 = 0x5a5a_0001;
const FRESH: u32 = 0x0123_4567;

#[test]
fn a_result_left_valid_is_cleared_before_the_post() {
    let bar = window();
    let mut immediate = Immediate::new(&bar, FRESH, Some(STALE));
    let _model = on_poll(move || immediate.step());
    let got = exchange(Regs::new(bar.base()), TO_0);
    assert_eq!(got, Some(FRESH), "the answer to an earlier verb was read as this one's");
    assert_eq!(bar.wrote32(ICOI as usize), TO_0, "the verb was never posted");
}

#[test]
fn a_result_that_will_not_clear_posts_nothing() {
    let bar = window();
    // A bare window keeps the IRV the driver writes to clear it.
    bar.present16(ICIS as usize, ICIS_IRV);
    assert_eq!(exchange(Regs::new(bar.base()), TO_0), None);
    assert_eq!(bar.wrote32(ICOI as usize), 0, "posted over a result still valid");
}

#[test]
fn a_controller_that_answers_is_answered_every_time() {
    let bar = window();
    let mut immediate = Immediate::new(&bar, FRESH, None);
    let _model = on_poll(move || immediate.step());
    for _ in 0..4 {
        assert_eq!(exchange(Regs::new(bar.base()), TO_0), Some(FRESH));
    }
}
