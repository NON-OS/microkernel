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

//! SCREENSHOT and ACCESSIBILITY_GET: what to capture into, and which
//! output's settings to read.

use super::read::{wide, words};
use crate::protocol::{ACCESSIBILITY_GET_REQ_LEN, SCREENSHOT_REQ_LEN};

/// SCREENSHOT: the surface handle the picture is written to.
pub fn screenshot(body: &[u8]) -> Option<u64> {
    let [lo, hi] = words::<2, SCREENSHOT_REQ_LEN>(body)?;
    Some(wide(lo, hi))
}

/// ACCESSIBILITY_GET: the output. Whether it exists is the caller's.
pub fn accessibility_get(body: &[u8]) -> Option<usize> {
    let [out] = words::<1, ACCESSIBILITY_GET_REQ_LEN>(body)?;
    Some(out as usize)
}
