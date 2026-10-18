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

//! Bringing the card up and proving the data path, and stopping the host.

use super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::error::{EmmcError, EmmcResult};
use super::super::mmc::ext_csd::EXT_CSD_LEN;
use super::super::sdhci::Host;
use super::super::text::Line;
use super::card::card;
use super::EmmcDisk;

impl<M: Mmio, C: Clock, L: Log> EmmcDisk<M, C, L> {
    /// Reset `host` for a bring-up (`Host::init`): Software Reset For All
    /// seen to finish, which ends any DMA, and ADMA2 offered. On failure the
    /// host is quiesced (reset, power off) before the error is returned.
    pub fn reset(mut host: Host<M, C, L>) -> EmmcResult<Host<M, C, L>> {
        if let Err(e) = host.init() {
            host.quiesce();
            return Err(e);
        }
        Ok(host)
    }

    /// Bring the card up on a host `reset` has readied, and read sector 0
    /// once to prove the data path. On failure the host is quiesced (reset,
    /// power off) before the error is returned, so the caller may give its
    /// DMA regions back.
    pub fn bring_up(mut host: Host<M, C, L>, data: DmaBuf) -> EmmcResult<Self> {
        if data.len < EXT_CSD_LEN {
            return Err(EmmcError::OutOfRange);
        }
        let card = match card(&mut host, data) {
            Ok(c) => c,
            Err(e) => {
                host.quiesce();
                return Err(e);
            }
        };
        let mut disk = Self { host, card, data };
        if let Err(e) = disk.transfer(0, 1, false) {
            disk.host.say(Line::new().s(b"sector 0 could not be read; not serving"));
            disk.host.quiesce();
            return Err(e);
        }
        Ok(disk)
    }

    /// Stop the host before its DMA regions go.
    pub fn quiesce(&mut self) {
        self.host.quiesce();
    }
}
