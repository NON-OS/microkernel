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

//! CMD9: the card's CSD, its size, and whether its structure agrees with
//! the capacity class its OCR claimed (sd/csd.rs).

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::mmc::cmds::{rca_arg, SEND_CSD};
use super::super::super::mmc::init::step::step;
use super::super::super::mmc::regs::{bits, r2};
use super::super::super::sdhci::{Cmd, Host, Resp};
use super::super::super::text::Line;
use super::super::regs::{csd_agrees, sd_sectors};

/// The card's size in sectors; refused when the CSD is a version the
/// driver does not serve, names an impossible block, or disagrees with how
/// the card is to be addressed.
pub(super) fn read_csd<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    rca: u16,
    sector_mode: bool,
) -> EmmcResult<u64> {
    let csd = r2(step(h, &Cmd::new(SEND_CSD, rca_arg(rca), Resp::R2), b"CMD9 SEND_CSD")?);
    let structure = bits(csd, 127, 126) as u64;
    let Some(sectors) = sd_sectors(csd) else {
        h.say(Line::new().s(b"SD csd structure ").dec(structure).s(b" not served"));
        return Err(EmmcError::NoCapacity);
    };
    if !csd_agrees(csd, sector_mode) {
        let mode: &[u8] = if sector_mode { b"block" } else { b"byte" };
        h.say(
            Line::new()
                .s(b"SD csd structure ")
                .dec(structure)
                .s(b" on a ")
                .s(mode)
                .s(b"-addressed card"),
        );
        return Err(EmmcError::NoCapacity);
    }
    Ok(sectors)
}
