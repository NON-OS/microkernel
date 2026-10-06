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

//! The model and serial IDENTIFY carries.

use super::super::mmc::Cid;
use super::maker::maker;
use super::{MODEL_MAX, SERIAL_LEN};

/// The model and serial IDENTIFY carries for an eMMC part.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Names {
    pub model: [u8; MODEL_MAX],
    pub model_len: usize,
    pub serial: [u8; SERIAL_LEN],
}

impl Names {
    pub fn model(&self) -> &[u8] {
        &self.model[..self.model_len]
    }
}

pub fn names(cid: &Cid) -> Names {
    let mut model = [0u8; MODEL_MAX];
    let mut n = 0;
    for &c in maker(cid.mid) {
        model[n] = c;
        n += 1;
    }
    let pnm: &[u8] = &cid.pnm;
    let end = pnm.iter().rposition(|&c| c != b' ' && c != 0).map_or(0, |i| i + 1);
    if end > 0 {
        model[n] = b' ';
        n += 1;
        for &c in &pnm[..end] {
            model[n] = if (0x20..0x7f).contains(&c) { c } else { b'?' };
            n += 1;
        }
    }
    let mut serial = [0u8; SERIAL_LEN];
    for (i, slot) in serial.iter_mut().enumerate() {
        let nibble = (cid.psn >> (28 - 4 * i)) & 0xf;
        *slot = b"0123456789ABCDEF"[nibble as usize];
    }
    Names { model, model_len: n, serial }
}
