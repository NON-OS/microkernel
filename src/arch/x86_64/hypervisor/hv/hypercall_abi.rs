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

/*
 * The hypercall interface's registers (TLFS 3): the control word in RCX,
 * the input in RDX (a page's address, or the value itself for a fast call),
 * the output page in R8, and the result in RAX.
 */

pub(super) const CALL_POST_MESSAGE: u16 = 0x005C;
pub(super) const CALL_SIGNAL_EVENT: u16 = 0x005D;

const FAST: u64 = 1 << 16;

pub const STATUS_SUCCESS: u16 = 0;
/// The host has no room for the message right now; try again.
pub const STATUS_INSUFFICIENT_BUFFERS: u16 = 0x13;

pub(super) const fn control(code: u16, fast: bool) -> u64 {
    code as u64 | if fast { FAST } else { 0 }
}

pub(super) const fn status(rax: u64) -> u16 {
    rax as u16
}

/// HvSignalEvent's one input: the connection and the flag in it.
pub(super) const fn signal_input(connection: u32, flag: u16) -> u64 {
    connection as u64 | (flag as u64) << 32
}

/// HvPostMessage's input page: connection, reserved, type, size, payload.
pub(super) const POST_HEADER: usize = 16;

pub(super) fn post_input(page: &mut [u8], connection: u32, ty: u32, payload: &[u8]) -> Option<()> {
    if payload.len() > super::message::PAYLOAD_MAX || ty == 0 {
        return None;
    }
    let out = page.get_mut(..POST_HEADER + payload.len())?;
    out[0..4].copy_from_slice(&connection.to_le_bytes());
    out[4..8].fill(0);
    out[8..12].copy_from_slice(&ty.to_le_bytes());
    out[12..16].copy_from_slice(&(payload.len() as u32).to_le_bytes());
    out[POST_HEADER..].copy_from_slice(payload);
    Some(())
}
