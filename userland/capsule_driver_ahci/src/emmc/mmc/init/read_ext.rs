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

//! The EXT_CSD of a version 4 card, read once at bring-up and logged.

use super::super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::super::error::EmmcResult;
use super::super::super::sdhci::Host;
use super::super::super::text::Line;
use super::super::cmds::RCA;
use super::super::ext_csd::ExtCsd;
use super::super::ops::{read_ext_csd, say_failed};
use super::super::regs::Csd;

/// The EXT_CSD when the CSD says the card has one (SPEC_VERS 4 or later).
pub(super) fn read_ext<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    csd: &Csd,
    data: DmaBuf,
) -> EmmcResult<Option<ExtCsd>> {
    let ext = if csd.spec_vers >= 4 {
        match read_ext_csd(h, RCA, data) {
            Ok(e) => Some(e),
            Err(e) => {
                say_failed(h, b"CMD8 SEND_EXT_CSD", e);
                return Err(e);
            }
        }
    } else {
        None
    };
    if let Some(e) = ext.as_ref() {
        h.say(
            Line::new()
                .s(b"ext_csd rev ")
                .dec(e.rev() as u64)
                .s(b" device type ")
                .hex(e.device_type() as u64)
                .s(b" partition config ")
                .hex(e.partition_config() as u64)
                .s(b" boot ")
                .dec(e.boot_sectors() / 2)
                .s(b" KiB x2 rpmb ")
                .dec(e.rpmb_sectors() / 2)
                .s(b" KiB cache ")
                .dec(e.cache_kib() as u64)
                .s(b" KiB")
                .s(if e.cache_on() { b" on" as &[u8] } else { b" off" }),
        );
    }
    Ok(ext)
}
