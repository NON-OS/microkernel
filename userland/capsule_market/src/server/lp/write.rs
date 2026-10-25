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
//! Writing length-prefixed fields and counts into a reply body.

use alloc::vec::Vec;

/// `bytes` with its length in front; None when the length does not fit the
/// u32 the wire gives it, so a reply never states a truncated length.
pub(crate) fn write_lp(out: &mut Vec<u8>, bytes: &[u8]) -> Option<()> {
    write_count(out, bytes.len())?;
    out.extend_from_slice(bytes);
    Some(())
}

/// A count as the wire's u32; None when it does not fit.
pub(crate) fn write_count(out: &mut Vec<u8>, n: usize) -> Option<()> {
    out.extend_from_slice(&u32::try_from(n).ok()?.to_le_bytes());
    Some(())
}
