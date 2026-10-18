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

//! A controller with LE Data Length Extension, brought up and connected.

use crate::sim::config::Config;
use crate::sim::events::{complete, status};
use crate::sim::link_events::{connected, CONNECT};
use crate::sim::stack::{stack, Stack};
use nonos_bt_wire::hci::data_length::{LE_READ_MAXIMUM_DATA_LENGTH, LE_SET_DATA_LENGTH};
use nonos_bt_wire::hci::opcode as op;

pub const DLE: u64 = 1 << 5;
pub const READ: u16 = LE_READ_MAXIMUM_DATA_LENGTH.0;
pub const SET: u16 = LE_SET_DATA_LENGTH.0;
pub const CREATE: u16 = op::LE_CREATE_CONNECTION.0;

fn max(v: [u16; 4]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}

/// A controller with the feature whose maximum is `m`, its Set Data Length
/// answered with `set_status`; brought up and connected on handle 0x40.
pub fn linked(features: u64, m: [u16; 4], set_status: u8, fail: Option<(u16, u8)>) -> Stack {
    let script = vec![
        (READ, vec![complete(1, READ, 0, &max(m))]),
        (CREATE, vec![status(0, 1, CREATE), connected(0, 0x40)]),
        (SET, vec![complete(1, SET, set_status, &[0x40, 0])]),
    ];
    let mut s = stack(Config { le_features: features, fail, ..Config::scripted(script) });
    s.bring_up().expect("brought up");
    assert_eq!(s.connect(&CONNECT).expect("connected").handle, 0x40);
    s
}

/// LE Set Data Length's parameters, if the host sent it.
pub fn asked(s: &mut Stack) -> Option<Vec<u8>> {
    let cmds = &s.transport().pipes().commands;
    cmds.iter().find(|(_, c, _)| c[..2] == SET.to_le_bytes()).map(|(_, c, _)| c[3..].to_vec())
}
