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
//! A UAS command's data on the second bulk pair, in pieces of one
//! controller page. A stalled data pipe is cleared on both sides and the
//! bytes that moved before it are what counts, as for BOT's data phase.

use nonos_uas::DataDir;

use super::types::{Data, Disk};
use crate::xhci::{
    bulk_in_on, bulk_out_on, control_no_data, reset_bulk_on, BULK_MAX, DATA_PAIR, E_PIPE,
};

/// CLEAR_FEATURE(ENDPOINT_HALT) to an endpoint.
const CLEAR_FEATURE: (u8, u8) = (0x02, 0x01);

pub(super) fn move_data(disk: &Disk, data: &mut Data) -> Result<usize, i32> {
    let (dir_in, len) = (matches!(data, Data::In(_)), data.len());
    let mut at = 0;
    while at < len {
        let n = (len - at).min(BULK_MAX);
        let moved = match data {
            Data::In(buf) => bulk_in_on(disk.xhci, disk.slot, DATA_PAIR, &mut buf[at..at + n]),
            Data::Out(buf) => bulk_out_on(disk.xhci, disk.slot, DATA_PAIR, &buf[at..at + n]),
            Data::None => return Ok(0),
        };
        match moved {
            Ok(m) if m == n => at += n,
            Ok(m) => return Ok(at + m.min(n)),
            Err(E_PIPE) => {
                reset_bulk_on(disk.xhci, disk.slot, DATA_PAIR, dir_in)?;
                let ep = if dir_in { disk.ep_in } else { disk.ep_out };
                control_no_data(disk.xhci, disk.slot, CLEAR_FEATURE, 0, ep as u16)?;
                return Ok(at);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(at)
}

/// Which way a command's data moves, as nonos_uas names it.
pub(super) fn dir_of(data: &Data) -> DataDir {
    match data {
        Data::None => DataDir::None,
        Data::In(_) => DataDir::In,
        Data::Out(_) => DataDir::Out,
    }
}
