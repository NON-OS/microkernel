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

//! The watchdog reads the link only when the answer turns on it: a frame
//! completed, or none in flight, settles it without a STATUS read, which
//! every send would otherwise pay for.

use std::{cell::Cell, rc::Rc};

use nonos_libc::entropy;

use super::tx_hang_tests::stalled;
use crate::constants::ctrl::STATUS_LU;
use crate::constants::regs::REG_STATUS;
use crate::regs::on_read;
use crate::server::tx_watch::check;

#[test]
fn the_link_is_read_only_while_frames_are_in_flight_and_none_completed() {
    let _turn = entropy(true);
    let (_part, _resets, mut d, _mem) = stalled(STATUS_LU);
    let reads = Rc::new(Cell::new(0u32));
    let n = Rc::clone(&reads);
    let _status = on_read(REG_STATUS, move |v| {
        n.set(n.get() + 1);
        v
    });
    assert!(!check(&mut d, true), "a completion is a sign of life");
    let clean = d.tx.clean;
    d.tx.clean = d.tx.tail;
    assert!(!check(&mut d, false), "nothing in flight");
    assert_eq!(reads.get(), 0, "the link was not read");
    d.tx.clean = clean;
    assert!(!check(&mut d, false), "in flight, inside the bound");
    assert_eq!(reads.get(), 1, "read once it decides");
}
