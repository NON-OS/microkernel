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
//! The information units the host sends on the command pipe: a Command
//! IU (01h) and a Task Management IU (05h). Every IU starts with its id, a
//! reserved byte and a big-endian tag.

use crate::command::ScsiCommand;

const COMMAND: u8 = 0x01;
const TASK_MANAGEMENT: u8 = 0x05;
pub const COMMAND_IU_LEN: usize = 32;
pub const TASK_IU_LEN: usize = 16;
/// Task management functions (as recalled, unverified).
pub const TMF_ABORT_TASK: u8 = 0x01;
pub const TMF_LOGICAL_UNIT_RESET: u8 = 0x08;

/// A LUN below 256 in SAM's single-level, peripheral form.
pub fn lun_bytes(lun: u8) -> [u8; 8] {
    [0, lun, 0, 0, 0, 0, 0, 0]
}

/// The Command IU for `cmd` under `tag`: task attribute simple, priority
/// 0, no additional CDB; `None` for a CDB not 6 to 16 bytes long, or tag 0.
pub fn command_iu(tag: u16, lun: u8, cmd: &dyn ScsiCommand) -> Option<[u8; COMMAND_IU_LEN]> {
    let cdb = cmd.cdb();
    if !(6..=16).contains(&cdb.len()) || tag == 0 {
        return None;
    }
    let mut iu = [0u8; COMMAND_IU_LEN];
    iu[0] = COMMAND;
    iu[2..4].copy_from_slice(&tag.to_be_bytes());
    iu[8..16].copy_from_slice(&lun_bytes(lun));
    iu[16..16 + cdb.len()].copy_from_slice(cdb);
    Some(iu)
}

/// A Task Management IU of `function` under `tag`, naming the task under
/// `managed` (for ABORT TASK; 0 otherwise).
pub fn task_iu(tag: u16, function: u8, managed: u16, lun: u8) -> [u8; TASK_IU_LEN] {
    let mut iu = [0u8; TASK_IU_LEN];
    iu[0] = TASK_MANAGEMENT;
    iu[2..4].copy_from_slice(&tag.to_be_bytes());
    iu[4] = function;
    iu[6..8].copy_from_slice(&managed.to_be_bytes());
    iu[8..16].copy_from_slice(&lun_bytes(lun));
    iu
}
