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

//! A bond record to its 48 octets, checked first: what is written is what
//! `decode` would accept. The output holds the keys; the caller overwrites
//! it once it has been handed on.

use super::layout::*;
use super::types::BondRecord;
use crate::error::BondError;

impl BondRecord {
    pub fn encode(&self) -> Result<[u8; RECORD_LEN], BondError> {
        self.check()?;
        let mut out = [0u8; RECORD_LEN];
        out[0] = VERSION;
        out[1] = SECURE_CONNECTIONS
            | if self.authenticated { AUTHENTICATED } else { 0 }
            | if self.irk.is_some() { HAS_IRK } else { 0 };
        out[2] = self.identity_type;
        out[ADDR..LTK].copy_from_slice(&self.identity);
        out[LTK..IRK].copy_from_slice(&self.ltk);
        if let Some(irk) = &self.irk {
            out[IRK..KEY_SIZE].copy_from_slice(irk);
        }
        out[KEY_SIZE] = self.key_size;
        Ok(out)
    }

    /// The rules a record must keep, as written and as read.
    pub(crate) fn check(&self) -> Result<(), BondError> {
        if !(7..=16).contains(&self.key_size) || self.ltk == [0; 16] {
            return Err(BondError::Invalid);
        }
        if self.irk == Some([0; 16]) {
            return Err(BondError::Invalid);
        }
        if !identity_allowed(self.identity_type, &self.identity) {
            return Err(BondError::Invalid);
        }
        Ok(())
    }
}

/// An identity is public, or static random: top bits 0b11 and the rest
/// neither all 0 nor all 1 (Vol 6, Part B, 1.3.2.1).
pub(super) fn identity_allowed(kind: u8, a: &[u8; 6]) -> bool {
    match kind {
        0 => true,
        1 => {
            let rest = u64::from_le_bytes([a[0], a[1], a[2], a[3], a[4], a[5] & 0x3f, 0, 0]);
            a[5] >> 6 == 0b11 && rest != 0 && rest != (1 << 46) - 1
        }
        _ => false,
    }
}
