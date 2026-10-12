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

//! The latency read through the power controller's mailbox.

use crate::engine::error::Error;
use crate::engine::pcode::latencies;
use crate::model::{Gen9, Polls};

#[test]
fn eight_levels_come_back_and_stop_at_the_first_without_one() {
    let mut m = Gen9::new(1);
    let l = latencies(&mut m, &Polls::new(40)).unwrap();
    assert_eq!(l, [2, 4, 8, 16, 24, 32, 48, 0]);
}

#[test]
fn a_mailbox_that_never_answers_times_out() {
    let mut m = Gen9::new(1);
    m.dead = true;
    m.set(crate::engine::pcode::MAILBOX, crate::engine::pcode::READY);
    assert_eq!(latencies(&mut m, &Polls::new(40)), Err(Error::Timeout));
}
