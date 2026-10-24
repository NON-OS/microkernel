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

//! The record's footer: how many entries, when and why the session ended,
//! the chain's final value, and the host keys' signature over all of it.

use crate::keys::{HybridSig, DIGEST_LEN, HYBRID_SIG_LEN};
use crate::{Reader, WireError};

pub const FOOTER_LEN: usize = 4 + 8 + 1 + DIGEST_LEN + HYBRID_SIG_LEN;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Footer<'a> {
    pub count: u32,
    pub end_ms: u64,
    pub end_reason: u8,
    pub chain: &'a [u8; DIGEST_LEN],
    pub sig: HybridSig<'a>,
}

impl<'a> Footer<'a> {
    pub fn read(r: &mut Reader<'a>) -> Result<Self, WireError> {
        let count = r.u32()?;
        let end_ms = r.u64()?;
        let end_reason = r.u8()?;
        if !(1..=6).contains(&end_reason) {
            return Err(WireError::Value);
        }
        Ok(Footer { count, end_ms, end_reason, chain: r.array()?, sig: HybridSig::read(r)? })
    }
}
