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

//! A controller that lost its state: a Hardware Error (Vol 4, Part E,
//! 7.7.16), or a Reset (7.3.2) completed that this host did not send, stops
//! the host with `Reset` wherever it meets it; while the host's own Reset
//! is waited for, both are the past and pass, and nothing from before it
//! reaches the caller.

use crate::sim::config::Config;
use crate::sim::events::complete;
use crate::sim::stack::stack;
use nonos_bt_hci::HciError;
use nonos_bt_wire::hci::opcode as op;

const HARDWARE_ERROR: [u8; 3] = [0x10, 1, 0x00];

#[test]
fn a_hardware_error_or_a_reset_no_one_sent_stops_the_host() {
    let reset = complete(1, op::RESET.0, 0, &[]);
    for lie in [HARDWARE_ERROR.to_vec(), reset] {
        let mut s = stack(Config::default());
        s.bring_up().unwrap();
        s.transport().pipes().send_event(&lie);
        assert_eq!(s.next_event(), Err(HciError::Reset), "between commands");
        s.transport().pipes().cfg.before = vec![lie];
        assert_eq!(s.command(op::READ_BD_ADDR, &[]), Err(HciError::Reset), "while one waits");
    }
}

#[test]
fn what_came_before_the_hosts_own_reset_is_cleared() {
    let mut s = stack(Config::default());
    s.transport().pipes().send_event(&HARDWARE_ERROR);
    s.transport().pipes().cfg.before = vec![HARDWARE_ERROR.to_vec()];
    s.command(op::RESET, &[]).unwrap();
    s.transport().pipes().cfg.before.clear();
    s.bring_up().unwrap();
    assert_eq!(s.next_event(), Ok(None));
}
