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

//! The PORT_LIST entry.

use super::super::sdhci::Snapshot;
use super::{PORT_ENTRY_LEN, PORT_KIND_EMMC};

pub fn port_entry(up: bool, snap: &Snapshot, ocr: u32, card_status: u32) -> [u8; PORT_ENTRY_LEN] {
    let mut o = [0u8; PORT_ENTRY_LEN];
    o[0] = 0;
    o[1] = 1;
    o[2] = up as u8;
    o[3] = PORT_KIND_EMMC;
    o[4..8].copy_from_slice(&snap.present.to_le_bytes());
    o[8..12].copy_from_slice(&ocr.to_le_bytes());
    o[12..16].copy_from_slice(&snap.int_status.to_le_bytes());
    o[16..20].copy_from_slice(&card_status.to_le_bytes());
    o[24..28].copy_from_slice(&(snap.int_status >> 16).to_le_bytes());
    o
}
