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

//! One boot keyboard report: the modifier bits, a reserved byte, and six
//! key slots. Pure, so the decode is proven on the host.

use crate::protocol::KEY_REPORT_LEN;

/// What one boot keyboard report names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootReport {
    pub modifiers: u8,
    pub keys: [u8; 6],
}

impl BootReport {
    /// The report in `raw`, or None unless it is exactly the eight bytes a
    /// boot keyboard sends. A short read does not say the missing slots are
    /// empty, so it must not release the keys they held, and a longer one
    /// is not a boot report at all; neither changes any state.
    pub fn parse(raw: &[u8]) -> Option<Self> {
        let report: &[u8; KEY_REPORT_LEN] = raw.try_into().ok()?;
        Some(Self {
            modifiers: report[0],
            keys: [report[2], report[3], report[4], report[5], report[6], report[7]],
        })
    }
}
