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

//! The host brought up over H4 exactly as over USB: the same standard
//! commands in the same order, each sent after the command indicator, and
//! the same controller read back, however the port cuts the octets.

use super::h4_port::H4Port;
use crate::sim::config::{Config, ADDR};
use nonos_bt_hci::uart::h4::H4Hci;
use nonos_bt_hci::Host;
use nonos_bt_wire::addr::BdAddr;
use nonos_bt_wire::hci::opcode as op;

#[test]
fn bring_up_over_h4_sends_each_command_after_its_indicator() {
    for chunk in [1, 2, 3, 7, 64] {
        let mut host = Host::new(H4Hci::new(H4Port::new(Config::default(), chunk), 27));
        let c = host.bring_up().unwrap();
        assert_eq!((c.addr, c.acl_len, c.acl_total), (BdAddr(ADDR), 27, 4), "chunk {chunk}");
        let written = &host.transport().port_mut().written;
        assert!(written.iter().all(|w| w[0] == 0x01), "the command indicator first");
        let ops: Vec<u16> = written.iter().map(|w| u16::from_le_bytes([w[1], w[2]])).collect();
        assert_eq!(
            ops[..4],
            [op::RESET.0, op::READ_LOCAL_FEATURES.0, op::READ_LOCAL_VERSION.0, op::READ_BD_ADDR.0]
        );
        assert_eq!(
            written[0],
            [0x01, 0x03, 0x0c, 0x00],
            "Reset: indicator, opcode 0x0c03, no parameters"
        );
    }
}

#[test]
fn acl_data_goes_out_after_its_indicator() {
    let mut h4 = H4Hci::new(H4Port::new(Config::default(), 64), 27);
    nonos_bt_hci::Transport::send_acl(&mut h4, &[0x40, 0x00, 0x01, 0x00, 0xaa]).unwrap();
    assert_eq!(h4.port_mut().written[0], [0x02, 0x40, 0x00, 0x01, 0x00, 0xaa]);
}
