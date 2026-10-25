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

//! A family as reserve() makes it.

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::family::Family;
use super::hist::HIST_WORDS;

impl Family {
    /// A reserved family: no guest yet, nothing counted, M at zero.
    pub(crate) fn fresh(
        sup: u32,
        id: ([u8; 32], u64, u8),
        trace: [u8; 32],
        salt: [u64; 4],
    ) -> Family {
        let (sup_meas, sup_caps, authority) = id;
        Family {
            sup,
            sup_meas,
            sup_caps,
            authority,
            enrolled: false,
            started: false,
            trace,
            rows: 0,
            next_seq: 0,
            hist: Box::new([0; HIST_WORDS]),
            saturated: false,
            salt,
            meas: [0; 32],
            n_meas: 0,
            overflow: false,
            list: Vec::new(),
            app_id: None,
            diag: [0; 32],
            open: Vec::new(),
        }
    }
}
