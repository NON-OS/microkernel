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

//! A command written into the command buffer and its slot's descriptor.

use super::super::plan::{CMD_BUFFER, CMD_TFDS, FIRST_TBS, FIRST_TB_SLOT, TFD_SIZE};
use super::super::region::{put16, put64, zero, Region};
use super::sizes::{FIRST_TB, WIDE_HDR};

/// The wide header: opcode, group, sequence, payload length, version.
pub(super) fn header(group: u8, cmd: u8, version: u8, sequence: u16, len: u16) -> [u8; WIDE_HDR] {
    let mut hdr = [0u8; WIDE_HDR];
    hdr[0] = cmd;
    hdr[1] = group;
    hdr[2..4].copy_from_slice(&sequence.to_le_bytes());
    hdr[4..6].copy_from_slice(&len.to_le_bytes());
    hdr[7] = version;
    hdr
}

/// Write the command into the command buffer and the descriptor of `slot`.
pub(super) fn write_command<R: Region + ?Sized>(
    ctrl: &R,
    slot: usize,
    hdr: &[u8; WIDE_HDR],
    payload: &[u8],
) -> bool {
    let total = WIDE_HDR + payload.len();
    if !ctrl.write(CMD_BUFFER, hdr) || !ctrl.write(CMD_BUFFER + WIDE_HDR, payload) {
        return false;
    }
    // The first 20 bytes (or the whole command, if shorter) in the slot's
    // own first-buffer slot.
    let tb0 = total.min(FIRST_TB);
    let mut first = [0u8; FIRST_TB];
    first[..WIDE_HDR].copy_from_slice(hdr);
    let from_payload = tb0 - WIDE_HDR;
    first[WIDE_HDR..tb0].copy_from_slice(&payload[..from_payload]);
    let first_at = FIRST_TBS + slot * FIRST_TB_SLOT;
    if !ctrl.write(first_at, &first[..tb0]) {
        return false;
    }
    let tfd = CMD_TFDS + slot * TFD_SIZE;
    if !zero(ctrl, tfd, TFD_SIZE) || !set_tb(ctrl, tfd, 0, tb0, ctrl.dev() + first_at as u64) {
        return false;
    }
    let mut tbs = 1u16;
    if total > tb0 {
        let rest = ctrl.dev() + (CMD_BUFFER + tb0) as u64;
        if !set_tb(ctrl, tfd, 1, total - tb0, rest) {
            return false;
        }
        tbs = 2;
    }
    put16(ctrl, tfd, tbs)
}

// Buffer `i` of the descriptor at `tfd`: `[len u16][addr u64]` after the count.
fn set_tb<R: Region + ?Sized>(ctrl: &R, tfd: usize, i: usize, len: usize, addr: u64) -> bool {
    let at = tfd + 2 + i * 10;
    put16(ctrl, at, len as u16) && put64(ctrl, at + 2, addr)
}
