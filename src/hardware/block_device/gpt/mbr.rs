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

//! Sector 0 of a GPT disk: the protective MBR (UEFI 2.10 5.2.3, as
//! recalled, unverified). A GPT counts only behind one: the 55h AAh
//! signature, and a partition record of type EEh that starts at LBA 1. A
//! disk later given an MBR of its own by another system may keep an old GPT
//! at LBA 1; that table is stale, and the other system's partitions are on
//! the sectors it names.

use super::bytes::le32;

const SECTOR: usize = 512;
const RECORDS: usize = 446;
const PROTECTIVE: u8 = 0xEE;

/// Whether `sector`, LBA 0, protects a GPT.
pub(crate) fn protects(sector: &[u8]) -> bool {
    if sector.len() < SECTOR || sector[510] != 0x55 || sector[511] != 0xAA {
        return false;
    }
    let records = sector[RECORDS..RECORDS + 64].chunks_exact(16);
    records.into_iter().any(|r| r[4] == PROTECTIVE && le32(r, 8) == 1)
}
