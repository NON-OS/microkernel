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

//! SEND_EXT_CSD: the 512-byte register read into the data buffer.

use super::super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::{Cmd, Data, Host, Resp};
use super::super::cmds::SEND_EXT_CSD;
use super::super::ext_csd::{ExtCsd, EXT_CSD_LEN};
use super::super::r1::errors;
use super::recover::recover;

/// One 512-byte EXT_CSD at 400 kHz on one line takes about 11 ms.
pub const EXT_CSD_MS: u64 = 1_000;

/// SEND_EXT_CSD into `data` and return a copy of it.
pub fn read_ext_csd<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    rca: u16,
    data: DmaBuf,
) -> EmmcResult<ExtCsd> {
    let c = Cmd {
        index: SEND_EXT_CSD,
        arg: 0,
        resp: Resp::R1,
        data: Some(Data { read: true, blocks: 1, multi: false, auto12: false, buf: data }),
        abort: false,
        wait_ms: EXT_CSD_MS,
    };
    match h.send(&c) {
        Ok(r) if errors(r[0]) != 0 => {
            recover(h, rca);
            return Err(EmmcError::Status { cmd: SEND_EXT_CSD, status: r[0] });
        }
        Ok(_) => {}
        Err(e) => {
            recover(h, rca);
            return Err(e);
        }
    }
    let mut raw = [0u8; EXT_CSD_LEN];
    if !data.get(0, &mut raw) {
        return Err(EmmcError::OutOfRange);
    }
    Ok(ExtCsd::new(raw))
}
