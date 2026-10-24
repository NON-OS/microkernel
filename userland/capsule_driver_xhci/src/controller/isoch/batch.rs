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

//! What an isoch OUT batch may hold.

use crate::protocol::{ISOCH_MAX, ISOCH_PACKETS_MAX};

/// Whether `lens` and `data` make a batch the endpoint can carry: 1 to
/// ISOCH_PACKETS_MAX packets, none empty or past one interval's payload,
/// their bytes `data`, at most ISOCH_MAX.
pub fn batch_fits(lens: &[u16], data: &[u8], esit: u32) -> bool {
    let sum: usize = lens.iter().map(|&l| l as usize).sum();
    (1..=ISOCH_PACKETS_MAX).contains(&lens.len())
        && lens.iter().all(|&l| l != 0 && l as u32 <= esit)
        && sum == data.len()
        && sum <= ISOCH_MAX
}
