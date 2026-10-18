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

//! What a model SD card is: OCR, version, capacity, CSD fields, identity.

#[derive(Debug, Clone)]
pub struct SdCfg {
    /// The voltage window, busy and CCS clear.
    pub ocr: u32,
    /// Version 2.00 or later, which answers CMD8.
    pub v2: bool,
    /// High capacity: blocks, not bytes; busy forever to a host without HCS.
    pub ccs: bool,
    /// ACMD41 polls with a window before the busy bit is set.
    pub ready_after: u32,
    /// CSD_STRUCTURE: 0 is version 1.0, 1 is 2.0, 2 is 3.0 (SDUC).
    pub csd_structure: u8,
    pub c_size: u32,
    pub c_size_mult: u8,
    pub read_bl_len: u8,
    /// Published on CMD3.
    pub rca: u16,
    pub mid: u8,
    pub oid: [u8; 2],
    pub name: [u8; 5],
    pub prv: u8,
    pub psn: u32,
    pub prg_ms: u64,
}

/// A 32 GB class SDHC card: (C_SIZE + 1) x 512 KiB.
impl Default for SdCfg {
    fn default() -> Self {
        Self {
            ocr: 0x00ff_8000,
            v2: true,
            ccs: true,
            ready_after: 3,
            csd_structure: 1,
            c_size: 0xe4df,
            c_size_mult: 0,
            read_bl_len: 9,
            rca: 0xb368,
            mid: 0x03,
            oid: *b"SD",
            name: *b"SL32G",
            prv: 0x80,
            psn: 0x1234_abcd,
            prg_ms: 3,
        }
    }
}
