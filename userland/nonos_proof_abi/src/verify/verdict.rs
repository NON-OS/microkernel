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

use crate::container::Kind;

/// What an accepted container says. "Only served calls" is `outside == 0`,
/// and that call is the verifier's, not this crate's.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Verdict {
    pub kind: Kind,
    pub window: u64,
    pub record_id: [u8; 32],
    pub app_id: [u8; 32],
    /// The device's tag in this listing's scope on `window`.
    pub tag: [u64; 4],
    /// N, every call the family made.
    pub calls: u64,
    /// U, the calls outside the declared set, each of them refused.
    pub outside: u64,
    /// |A|, how many buckets the declared set lets succeed.
    pub declared: u32,
    pub anchor: [u8; 32],
}
