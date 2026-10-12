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

//! A controller that answers the standard bring-up and scan commands the
//! way the cited sections describe, with the knobs the proofs turn.

use super::config::{Config, ADDR};
use super::events::complete;
use nonos_bt_wire::hci::command::parse_command;
use nonos_bt_wire::hci::opcode as op;

/// The events one command packet draws.
pub fn respond(cfg: &Config, packet: &[u8]) -> Vec<Vec<u8>> {
    let (opcode, _) = parse_command(packet).expect("the host sent a malformed command");
    let o = opcode.0;
    if cfg.silent {
        return Vec::new();
    }
    let mut out = cfg.before.clone();
    if let Some((f, s)) = cfg.fail.filter(|&(f, _)| f == o) {
        out.push(complete(1, f, s, &[]));
        return out;
    }
    let ret: Vec<u8> = match opcode {
        op::READ_LOCAL_FEATURES => vec![0, 0, 0, 0, if cfg.le { 0x40 } else { 0 }, 0, 0, 0],
        op::READ_LOCAL_VERSION => vec![0x0d, 0x34, 0x12, 0x0d, 0x5f, 0x00, 0x78, 0x56],
        op::READ_BD_ADDR => ADDR.to_vec(),
        op::LE_READ_BUFFER_SIZE => {
            let [l0, l1] = cfg.le_buffers.0.to_le_bytes();
            vec![l0, l1, cfg.le_buffers.1]
        }
        op::READ_BUFFER_SIZE => {
            let ([l0, l1], [n0, n1]) = (cfg.buffers.0.to_le_bytes(), cfg.buffers.1.to_le_bytes());
            vec![l0, l1, 64, n0, n1, 0, 0]
        }
        _ => Vec::new(),
    };
    out.push(complete(1, o, 0, &ret));
    out
}
