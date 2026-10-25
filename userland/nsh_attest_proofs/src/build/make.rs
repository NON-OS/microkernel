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

//! The PCR values a spec gives: the log's for those firmware extends, the
//! release's for 8, 9 and 14, then the spec's overrides.

use nsh_attest::consts::QUOTED_PCRS_V4;

use super::Spec;

type Over = [(u8, [u8; 32])];

fn pick(i: u8, over: &Over, base: [u8; 32]) -> [u8; 32] {
    over.iter().rev().find(|o| o.0 == i).map_or(base, |o| o.1)
}

impl Spec {
    /// What the document carries, in `QUOTED_PCRS_V4` order.
    pub fn values(&self) -> Vec<[u8; 32]> {
        let r = &self.releases[0];
        let base = |i: u8| match i {
            8 => r.pcr8,
            9 => r.pcr9,
            _ => self.log.pcrs[i as usize],
        };
        QUOTED_PCRS_V4.iter().map(|&i| pick(i, &self.pcr, base(i))).collect()
    }

    /// What the quote's digest covers.
    pub fn quoted_values(&self) -> Vec<[u8; 32]> {
        let v = self.values();
        QUOTED_PCRS_V4.iter().zip(v).map(|(&i, v)| pick(i, &self.quoted, v)).collect()
    }
}
