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

//! Bringing the simulated controller up: the standard commands in order,
//! what is read back, and each way bring-up stops.

use crate::sim::config::{Config, ADDR};
use crate::sim::stack::{opcodes, stack, stack_with_budget};
use nonos_bt_hci::HciError;
use nonos_bt_wire::addr::BdAddr;
use nonos_bt_wire::hci::opcode as op;

#[test]
fn bring_up_sends_the_standard_commands_and_reads_the_controller() {
    let mut s = stack(Config::default());
    let c = s.bring_up().unwrap();
    let want = [op::RESET, op::READ_LOCAL_FEATURES, op::READ_LOCAL_VERSION, op::READ_BD_ADDR];
    let tail = [op::LE_READ_LOCAL_FEATURES, op::LE_READ_BUFFER_SIZE];
    let tail = [&tail[..], &[op::SET_EVENT_MASK, op::LE_SET_EVENT_MASK]].concat();
    let want: Vec<u16> = want.iter().chain(&tail).map(|o| o.0).collect();
    assert_eq!(opcodes(&mut s), want);
    assert_eq!((c.hci_version, c.hci_revision, c.lmp_version), (0x0d, 0x1234, 0x0d));
    assert_eq!((c.company, c.lmp_subversion, c.addr), (0x005f, 0x5678, BdAddr(ADDR)));
    assert_eq!((c.acl_len, c.acl_total, s.acl_free()), (27, 4, 4));
    let cmds = &s.transport().pipes().commands;
    assert_eq!(cmds[6].1, [0x01, 0x0c, 8, 0x90, 0x80, 0, 0, 0, 0x80, 0, 0x20]);
    assert_eq!(cmds[7].1, [0x01, 0x20, 8, 0x1f, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(c.le_features, 0);
}

#[test]
fn shared_buffers_are_read_with_read_buffer_size() {
    let cfg = Config { le_buffers: (0, 0), buffers: (310, 10), ..Config::default() };
    let mut s = stack(cfg);
    let c = s.bring_up().unwrap();
    assert!(opcodes(&mut s).contains(&op::READ_BUFFER_SIZE.0));
    assert_eq!((c.acl_len, c.acl_total), (310, 10));
    let none = Config { le_buffers: (0, 0), buffers: (0, 0), ..Config::default() };
    assert_eq!(stack(none).bring_up(), Err(HciError::NoBuffers));
}

#[test]
fn bring_up_stops_without_le_on_a_failed_command_and_on_silence() {
    let no_le = Config { le: false, ..Config::default() };
    assert_eq!(stack(no_le).bring_up(), Err(HciError::NoLe));
    let fail = Config { fail: Some((op::RESET.0, 0x01)), ..Config::default() };
    assert_eq!(stack(fail).bring_up(), Err(HciError::Status(op::RESET, 0x01)));
    let mut silent = stack_with_budget(Config { silent: true, ..Config::default() }, 50);
    assert_eq!(silent.bring_up(), Err(HciError::Timeout(op::RESET)));
    assert_eq!(silent.transport().pipes().pauses, 50);
}
