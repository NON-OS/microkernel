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

//! The line a port leaves when its signature names a device the driver
//! does not serve, so a disk behind a port multiplier is never silent.

use crate::constants::regs::{SIG_ATAPI, SIG_PM, SIG_SEMB};
use crate::log::Line;

/// The signatures are the ones libata's ata_dev_classify tells apart. A
/// port multiplier is refused: the driver sends no PMP commands, so the
/// disks behind one are not reachable, and the log says so rather than
/// leaving the port out without a word.
pub(super) fn say_skipped(index: u8, sig: u32) {
    let kind: &[u8] = match sig {
        SIG_PM => b"port multiplier, not supported; disks behind it are not served",
        SIG_ATAPI => b"ATAPI device, not a disk",
        SIG_SEMB => b"enclosure bridge, not a disk",
        _ => b"not a disk",
    };
    Line::new()
        .text(b"port ")
        .num(u64::from(index))
        .text(b": ")
        .text(kind)
        .text(b" SIG ")
        .hex(sig)
        .send();
}
