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

//! The restore scan's state (v1.4.56): where it is, the batch the Shield
//! derived, and what it found.

use alloc::vec::Vec;

use super::landed_held::Held;

pub const GAP_LIMIT: u32 = 64;
pub const MAX_HANDED: u32 = 4096;
/// Tries for one address before it is passed over, neither empty nor found.
pub(super) const TRIES: u8 = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ScanState {
    #[default]
    Idle,
    /// The Shield is deriving the next batch, from this index (0: its count).
    Deriving(u32),
    Reading,
    /// Past GAP_LIMIT empty indices, or at MAX_HANDED.
    Stopped,
}

#[derive(Default)]
pub struct Scan {
    pub state: ScanState,
    pub batch: Vec<(u32, [u8; 20])>,
    pub at: usize,
    pub tries: u8,
    pub empty_run: u32,
    pub found: Vec<(u32, [u8; 20], Held)>,
}
