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

//! What the rig recorded, and checks on the frames it saw.

use nonos_wifi_core::dot11::ccmp::{decrypt, CCMP_HDR_LEN, MIC_LEN};
use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;

use super::*;

impl Rig {
    /// Every command since boot, as (group, opcode).
    pub fn commands(&self) -> Vec<(u8, u8)> {
        self.model.s.borrow().commands.iter().map(|c| (c.group, c.cmd)).collect()
    }

    pub fn command_payloads(&self, group: u8, cmd: u8) -> Vec<Vec<u8>> {
        self.model.s.borrow().commands.iter().filter(|c| (c.group, c.cmd) == (group, cmd)).map(|c| c.payload.clone()).collect()
    }

    /// Every frame the station queued.
    pub fn sent(&self) -> Vec<crate::gen3_model::Sent> {
        self.model.s.borrow().txq.as_ref().unwrap().sent.clone()
    }

    pub fn bc_mismatch(&self) -> u32 {
        self.model.s.borrow().txq.as_ref().unwrap().bc_mismatch
    }
}

/// Whether `f` is an EAPOL frame to the access point in the clear.
pub fn is_clear_eapol(f: &[u8]) -> bool {
    f[0] == 0x08 && f[1] & 0x40 == 0 && f.len() >= 32 && f[24..30] == [0xAA, 0xAA, 0x03, 0, 0, 0] && f[30..32] == [0x88, 0x8E]
}

/// The station's protected data frame decrypts under `tk` to a body.
pub fn decrypts(f: &[u8], tk: &[u8; 16]) -> bool {
    f[1] & 0x40 != 0 && decrypt(f, tk).is_some() && f.len() > MAC_HEADER_LEN + CCMP_HDR_LEN + MIC_LEN
}
