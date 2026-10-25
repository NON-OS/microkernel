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

//! Whether the controller still knows a link: Read RSSI (Vol 4, Part E,
//! 7.5.4) answered, or refused as Unknown Connection Identifier.

use crate::sim::config::Config;
use crate::sim::stack::stack;
use nonos_bt_hci::host::READ_RSSI;
use nonos_bt_hci::HciError;

#[test]
fn a_link_the_controller_no_longer_has_is_told_apart() {
    let mut s = stack(Config::default());
    s.bring_up().unwrap();
    assert_eq!(s.link_known(0x0040), Ok(true));
    s.transport().pipes().cfg.fail = Some((READ_RSSI.0, 0x02));
    assert_eq!(s.link_known(0x0040), Ok(false));
    s.transport().pipes().cfg.fail = Some((READ_RSSI.0, 0x01));
    assert_eq!(s.link_known(0x0040), Err(HciError::Status(READ_RSSI, 0x01)));
    let sent = &s.transport().pipes().commands;
    assert_eq!(sent.last().unwrap().1, [0x05, 0x14, 2, 0x40, 0x00]);
}
