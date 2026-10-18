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

//! The NTB parameter structure GET_NTB_PARAMETERS returns (NCM 1.0, 6.2.1,
//! table 6-3), with the fields a host that receives and sends 16-bit NTBs
//! uses.

/// The structure's length, which its own wLength gives too.
pub const NTB_PARAMETERS_LEN: usize = 0x1C;
/// bmNtbFormatsSupported bit 1: the device also handles 32-bit NTBs.
const NTB32_SUPPORTED: u16 = 1 << 1;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct NtbParams {
    pub formats: u16,
    pub in_max: u32,
    pub out_max: u32,
    pub out_divisor: u16,
    pub out_remainder: u16,
    pub out_alignment: u16,
}

impl NtbParams {
    pub fn ntb32(&self) -> bool {
        self.formats & NTB32_SUPPORTED != 0
    }
}

/// The structure in `raw`, the bytes the device sent; `None` when it sent
/// less of it, or its own wLength says it is shorter than the fields read.
pub fn parse_params(raw: &[u8]) -> Option<NtbParams> {
    let w = |at: usize| u16::from_le_bytes([raw[at], raw[at + 1]]);
    if raw.len() < NTB_PARAMETERS_LEN || (w(0) as usize) < NTB_PARAMETERS_LEN {
        return None;
    }
    let d = |at: usize| u32::from_le_bytes([raw[at], raw[at + 1], raw[at + 2], raw[at + 3]]);
    Some(NtbParams {
        formats: w(2),
        in_max: d(4),
        out_max: d(16),
        out_divisor: w(20),
        out_remainder: w(22),
        out_alignment: w(24),
    })
}
