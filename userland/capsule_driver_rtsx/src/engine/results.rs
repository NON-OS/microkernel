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

//! The bytes the chip wrote back over the command buffer: one for each
//! read or check entry, in order (rtsx_pci_get_cmd_data).

use crate::setup::Driver;

/// The most a command asks for: the check, sixteen R2 bytes, SD_STAT1.
pub const RESULT_BYTES: usize = 18;

pub fn results(drv: &Driver) -> [u8; RESULT_BYTES] {
    let mut out = [0u8; RESULT_BYTES];
    for (i, b) in out.iter_mut().enumerate() {
        *b = drv.resv.read_u8(i as u64);
    }
    out
}
