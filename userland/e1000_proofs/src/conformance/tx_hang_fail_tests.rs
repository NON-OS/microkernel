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

//! A hung transmitter whose reset fails: the step is named, the frames stay
//! where they were, and the watchdog tries again once it falls due again.

use nonos_libc::{entropy, logged};

use super::tx_hang_tests::stalled;
use crate::constants::status::STATUS_LU;
use crate::server::tx_watch::check;

#[test]
fn a_reset_that_fails_is_named_and_tried_again_later() {
    let _turn = entropy(true);
    let (_bar, part, mut d, _mem) = stalled(STATUS_LU);
    drop(part);
    assert!(check(&mut d, false), "the hang is acted on");
    let line = "driver.e1000: tx hang, reset: CTRL.RST did not self-clear\n";
    assert_eq!(logged().last().map(String::as_str), Some(line));
    assert_eq!(d.tx.tail, 3, "nothing restarted on a part that did not reset");
    assert!(!d.tx_watch.expired(), "the next try waits a whole bound");
    assert!(!check(&mut d, false));
}
