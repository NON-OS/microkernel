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

//! The data phase of one BOT command, in pieces of one controller page.

use super::recover::clear_halt;
use super::types::{Data, Disk};
use crate::xhci::{bulk_in, bulk_out, BULK_MAX, E_PIPE};

/// Move the data phase. A device that stalls it ends it early; the CSW
/// residue then says how much moved.
pub(super) fn data_phase(disk: &Disk, data: Data) -> Result<(), i32> {
    let (dir_in, len) = (matches!(data, Data::In(_)), data.len());
    let mut at = 0;
    let mut data = data;
    while at < len {
        let n = (len - at).min(BULK_MAX);
        let moved = match &mut data {
            Data::In(buf) => bulk_in(disk.xhci, disk.slot, &mut buf[at..at + n]),
            Data::Out(buf) => bulk_out(disk.xhci, disk.slot, &buf[at..at + n]),
            Data::None => return Ok(()),
        };
        match moved {
            Ok(m) if m == n => at += n,
            Ok(_) => return Ok(()),
            Err(E_PIPE) => return clear_halt(disk, dir_in),
            Err(e) => return Err(e),
        }
    }
    Ok(())
}
