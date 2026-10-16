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

//! One device management question to unit 0, no data, asked per policy.

use super::once::once;
use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::{NOP_MS, NOP_TRIES, QUERY_MS, QUERY_TRIES};

#[derive(Clone, Copy)]
pub enum Ask {
    /// Is a device answering at the UTP layer?
    Nop,
    /// A flag read or set; the answer is its value now.
    Flag { func: u8, op: u8, idn: u8 },
}

/// Tries, the bound on each, and whether a silent try ends the question: a
/// device that lets a NOP OUT go unanswered is not about to answer the
/// next, while a query may be late behind the device's own work.
fn policy(a: Ask) -> (usize, u64, bool) {
    match a {
        Ask::Nop => (NOP_TRIES, NOP_MS, true),
        Ask::Flag { .. } => (QUERY_TRIES, QUERY_MS, false),
    }
}

pub fn ask<H: Hw>(hw: &H, c: &Ctx, a: Ask) -> Result<bool, UfsError> {
    let (tries, ms, silence_ends) = policy(a);
    let mut last = UfsError::Timeout;
    for _ in 0..tries {
        match once(hw, c, a, ms) {
            Ok(v) => return Ok(v),
            Err(UfsError::Timeout) if silence_ends => return Err(UfsError::Timeout),
            Err(e) => last = e,
        }
    }
    Err(last)
}
