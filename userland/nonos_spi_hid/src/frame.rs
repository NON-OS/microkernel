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
//! What goes on the wire: an output report wrapped in its write frame, and
//! the read frames that fetch an input report's header and then its body.
//! Opcodes and addresses are the firmware's (_DSM), passed in.

use alloc::vec::Vec;

use super::facts::FRAME_HEAD;

/// The firmware's parameters for this device's transactions.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Params {
    pub read_opcode: u8,
    pub write_opcode: u8,
    pub input_address: u32,
    pub output_address: u32,
}

fn head(opcode: u8, address: u32) -> [u8; FRAME_HEAD] {
    let a = address.to_be_bytes();
    [opcode, a[1], a[2], a[3]]
}

/// An output report of `kind` and `id` with `content`, framed for writing.
pub fn write_frame(p: &Params, kind: u8, id: u8, content: &[u8]) -> Option<Vec<u8>> {
    let len = u16::try_from(content.len()).ok()?;
    let mut f = Vec::with_capacity(FRAME_HEAD + 4 + content.len());
    f.extend(head(p.write_opcode, p.output_address));
    f.push(kind);
    f.extend(len.to_le_bytes());
    f.push(id);
    f.extend_from_slice(content);
    Some(f)
}

/// A read of `len` bytes: the frame head, then as many bytes clocked in.
/// The device's answer is what came back after the head.
pub fn read_frame(p: &Params, len: usize) -> Vec<u8> {
    let mut f = Vec::with_capacity(FRAME_HEAD + len);
    f.extend(head(p.read_opcode, p.input_address));
    f.resize(FRAME_HEAD + len, 0);
    f
}
