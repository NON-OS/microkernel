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

//! The ring's three record operations and its list.

use crate::ring_sim::Ring;

const EINVAL: i32 = -22;
const ENOENT: i32 = -2;
const EACCES: i32 = -13;
const ENOSPC: i32 = -28;
/// `MAX_KEYS_PER_OWNER` in `capsule_keyring/src/store/types`.
const PER_OWNER: usize = 16;

impl Ring {
    pub(crate) fn store(&mut self, owner: u32, p: &[u8]) -> Result<Vec<u8>, i32> {
        if p.len() < 23 || !(0..=11).contains(&p[20]) {
            return Err(EINVAL);
        }
        let len = u16::from_le_bytes([p[21], p[22]]) as usize;
        if p.len() != 23 + len || len == 0 {
            return Err(EINVAL);
        }
        if self.records.values().filter(|r| r.0 == owner).count() >= PER_OWNER {
            return Err(ENOSPC);
        }
        let id = self.next;
        self.next += 1;
        self.records.insert(id, (owner, p[20], p[23..].to_vec()));
        Ok(id.to_le_bytes().to_vec())
    }

    pub(crate) fn by_id(&mut self, op: u16, owner: u32, id: u32) -> Result<Vec<u8>, i32> {
        let r = self.records.get(&id).ok_or(ENOENT)?;
        if r.0 != owner {
            return Err(EACCES);
        }
        match op {
            2 => Ok(r.2.clone()),
            _ => self.records.remove(&id).map(|_| Vec::new()).ok_or(ENOENT),
        }
    }

    pub(crate) fn list(&self, owner: u32, kind: u8) -> Vec<u8> {
        let ids: Vec<u32> = self
            .records
            .iter()
            .filter(|(_, r)| r.0 == owner && r.1 == kind)
            .map(|(i, _)| *i)
            .collect();
        let mut out = (ids.len() as u32).to_le_bytes().to_vec();
        ids.iter().for_each(|i| out.extend_from_slice(&i.to_le_bytes()));
        out
    }
}
