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

//! The HDMI Forum EDID extension override data block (HDMI 2.1 section
//! 10.3.6): a sink with more than one extension sets the base block's
//! count to 1 and puts the true count here, the first data block of the
//! first CTA extension, so older readers still see a valid structure.

use super::blocks::{Cta, TAG_EXTENDED};
use crate::blocks::{block, checksum_ok};

/// Extended tag 0x78.
pub const EXT_EEODB: u8 = 0x78;

/// The extension count the block gives, if block 1 carries one.
pub fn count(raw: &[u8]) -> Option<usize> {
    let b = block(raw, 1).filter(|b| checksum_ok(*b))?;
    let first = Cta::new(b)?.data_blocks().next()?;
    match (first.tag, first.extended, first.body.first()) {
        (TAG_EXTENDED, Some(EXT_EEODB), Some(&n)) if n >= 1 => Some(n as usize),
        _ => None,
    }
}
