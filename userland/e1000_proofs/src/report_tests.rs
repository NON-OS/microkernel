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

//! The driver's console lines: what a photo of `log` shows once the card is
//! up, and a link line only when the link changes.

use nonos_libc::logged;

use crate::constants::status::{STATUS_FD, STATUS_LU};
use crate::report;

#[test]
fn bring_up_logs_ctrl_and_status_in_hex() {
    report::up(0x0014_0248, 0x0000_0083);
    assert_eq!(logged(), ["driver.e1000: up, ctrl=00140248 status=00000083\n"]);
}

/// The stack polls the link every few hundred milliseconds; only the
/// changes reach the log, with the speed from STATUS bits 7:6.
#[test]
fn the_link_is_logged_on_each_change_with_speed_and_duplex() {
    let gig_full = STATUS_LU | STATUS_FD | (2 << 6);
    report::link(0);
    report::link(0);
    report::link(gig_full);
    report::link(gig_full);
    report::link(STATUS_LU | (1 << 6));
    report::link(0);
    report::link(STATUS_LU);
    assert_eq!(
        logged(),
        [
            "driver.e1000: link down\n",
            "driver.e1000: link up 1000 Mb/s full duplex\n",
            "driver.e1000: link down\n",
            "driver.e1000: link up 10 Mb/s half duplex\n",
        ]
    );
}
