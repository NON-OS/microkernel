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

//! FETCH's reply body: total u32 | offset u32 | the bytes from offset, at
//! most PAGE_LEN, inside total.

use alloc::vec::Vec;

use crate::fetch::{MAX_RELEASE, PAGE_LEN};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Page {
    pub total: u32,
    pub offset: u32,
    pub bytes: Vec<u8>,
}

impl Page {
    /// The page of `release` at `offset`; None past its end.
    pub fn of(release: &[u8], offset: u32) -> Option<Self> {
        let start = offset as usize;
        let rest = release.get(start..)?;
        if release.len() > MAX_RELEASE || (start == release.len() && start != 0) {
            return None;
        }
        let bytes = rest.get(..rest.len().min(PAGE_LEN))?.to_vec();
        Some(Page { total: u32::try_from(release.len()).ok()?, offset, bytes })
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.bytes.len().saturating_add(8));
        out.extend_from_slice(&self.total.to_le_bytes());
        out.extend_from_slice(&self.offset.to_le_bytes());
        out.extend_from_slice(&self.bytes);
        out
    }

    pub fn parse(b: &[u8]) -> Option<Self> {
        let total = u32::from_le_bytes(b.get(..4)?.try_into().ok()?);
        let offset = u32::from_le_bytes(b.get(4..8)?.try_into().ok()?);
        let bytes = b.get(8..)?.to_vec();
        let end = (offset as usize).checked_add(bytes.len())?;
        let inside = end <= total as usize && total as usize <= MAX_RELEASE;
        (inside && bytes.len() <= PAGE_LEN).then_some(Page { total, offset, bytes })
    }
}
