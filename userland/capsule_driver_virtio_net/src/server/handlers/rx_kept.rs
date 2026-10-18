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

//! The last batch handed out, kept so a lost reply can be asked for again.

extern crate alloc;

use alloc::vec::Vec;

use crate::scrub::scrub;

/// The batch, and to whom, kept until its number moves on.
pub struct Kept {
    pub pid: u32,
    pub seq: u32,
    pub body: Vec<u8>,
}

/// Drop the kept batch, wiping it: its frames were delivered.
pub fn forget(kept: &mut Option<Kept>) {
    if let Some(mut k) = kept.take() {
        scrub(&mut k.body);
    }
}
