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

//! This host's own identity resolving key, kept as 17 octets: version 1,
//! then the key most significant octet first. It is drawn once and kept, so
//! a peer bonded in one boot can still resolve this host in the next
//! (Vol 3, Part C, 10.7.1).

use crate::error::BondError;
use crate::wipe::wipe;

pub const LOCAL_LEN: usize = 17;
const VERSION: u8 = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalIrk([u8; 16]);

impl LocalIrk {
    /// A drawn key; an all-zero one is refused, as it means no draw took
    /// place.
    pub fn new(irk: [u8; 16]) -> Result<Self, BondError> {
        if irk == [0; 16] {
            return Err(BondError::Invalid);
        }
        Ok(Self(irk))
    }

    pub fn key(&self) -> &[u8; 16] {
        &self.0
    }

    pub fn encode(&self) -> [u8; LOCAL_LEN] {
        let mut out = [0u8; LOCAL_LEN];
        out[0] = VERSION;
        out[1..].copy_from_slice(&self.0);
        out
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, BondError> {
        let b: &[u8; LOCAL_LEN] = bytes.try_into().map_err(|_| BondError::Length)?;
        if b[0] != VERSION {
            return Err(BondError::Version);
        }
        let mut irk = [0u8; 16];
        irk.copy_from_slice(&b[1..]);
        Self::new(irk)
    }
}

impl Drop for LocalIrk {
    fn drop(&mut self) {
        wipe(&mut self.0);
    }
}
