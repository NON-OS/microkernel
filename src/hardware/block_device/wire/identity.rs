// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The IDENTIFY reply body (op 6), held to the wire's rules before anything
//! in it is shown or used as an erase-confirm word. Pure, so kernel_proofs
//! runs it on the host.

use super::header::IDENTIFY_LEN;

const MODEL_MAX: usize = 40;
const SERIAL_MAX: usize = 20;
const MODEL_AT: usize = 4;
const SERIAL_AT: usize = 44;

/// What a block-wire disk says it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Identity {
    model: [u8; MODEL_MAX],
    model_len: usize,
    serial: [u8; SERIAL_MAX],
    serial_len: usize,
    /// 0 unknown, 1 eMMC, 2 SD, 3 UFS, 4 card reader.
    pub medium: u8,
}

impl Identity {
    pub fn model(&self) -> &[u8] {
        &self.model[..self.model_len]
    }

    pub fn serial(&self) -> &[u8] {
        &self.serial[..self.serial_len]
    }
}

/// None for a body that is not exactly the wire's 64 bytes, names a length
/// past its field, sets the reserved byte, holds anything but 0x20..=0x7E
/// inside a length, or anything but zero past one.
pub(crate) fn decode_identity(body: &[u8]) -> Option<Identity> {
    if body.len() != IDENTIFY_LEN || body[3] != 0 {
        return None;
    }
    let (model_len, serial_len) = (body[0] as usize, body[1] as usize);
    let model = field::<MODEL_MAX>(&body[MODEL_AT..MODEL_AT + MODEL_MAX], model_len)?;
    let serial = field::<SERIAL_MAX>(&body[SERIAL_AT..SERIAL_AT + SERIAL_MAX], serial_len)?;
    Some(Identity { model, model_len, serial, serial_len, medium: body[2] })
}

fn field<const N: usize>(raw: &[u8], len: usize) -> Option<[u8; N]> {
    if len > N {
        return None;
    }
    let (text, pad) = raw.split_at(len);
    if !text.iter().all(|c| (0x20..=0x7e).contains(c)) || pad.iter().any(|&c| c != 0) {
        return None;
    }
    let mut out = [0u8; N];
    out.copy_from_slice(raw);
    Some(out)
}
