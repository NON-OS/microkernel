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

//! A bonded peer. Pairing here is LE Secure Connections only, so there is
//! no EDIV or Rand to keep: both are zero (Vol 3, Part H, 2.4.4). The keys
//! are overwritten when the record is dropped.

use crate::wipe::wipe;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BondRecord {
    /// Passkey Entry or Numeric Comparison protected the pairing; Just Works
    /// did not (Vol 3, Part H, 2.3.5.2).
    pub authenticated: bool,
    /// 0 public, 1 static random (Vol 3, Part H, 3.6.5).
    pub identity_type: u8,
    /// Least significant octet first.
    pub identity: [u8; 6],
    pub ltk: [u8; 16],
    pub irk: Option<[u8; 16]>,
    pub key_size: u8,
}

impl Drop for BondRecord {
    fn drop(&mut self) {
        wipe(&mut self.ltk);
        if let Some(irk) = self.irk.as_mut() {
            wipe(irk);
        }
    }
}
