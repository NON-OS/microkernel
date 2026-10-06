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

//! The widest bus width whose EXT_CSD reads back intact.

use super::super::super::env::{Clock, DmaBuf, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::Host;
use super::super::super::text::Line;
use super::super::card::Card;
use super::super::ext_csd::{bus_width_value, BUS_WIDTH};
use super::super::ops::{read_ext_csd, recover, say_failed, switch};
use super::select::widths;

pub(super) fn bus_width<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &mut Card,
    data: DmaBuf,
) -> EmmcResult<()> {
    let Some(reference) = card.ext else {
        return Ok(());
    };
    for &w in widths(h.can_bus8()) {
        if let Err(e) = switch(h, card, BUS_WIDTH, bus_width_value(w)) {
            say_failed(h, width_name(w), e);
            if !recover(h, card.rca) {
                return Err(e);
            }
            continue;
        }
        h.set_width(w);
        match read_ext_csd(h, card.rca, data) {
            Ok(again) if reference.bus_test_same(&again) => {
                card.width = w;
                h.say(Line::new().s(b"bus width ").dec(w as u64).s(b" bits"));
                return Ok(());
            }
            Ok(_) => h.say(Line::new().s(width_name(w)).s(b": EXT_CSD read back differs")),
            Err(e) => say_failed(h, width_name(w), e),
        }
        if !recover(h, card.rca) {
            return Err(EmmcError::BusWidth);
        }
    }
    h.set_width(1);
    // The card is at whatever width it last accepted; one line is set on
    // the card only through the loop's last entry, which failed.
    h.say(Line::new().s(b"no bus width reads EXT_CSD back intact"));
    Err(EmmcError::BusWidth)
}

fn width_name(w: u8) -> &'static [u8] {
    match w {
        8 => b"CMD6 BUS_WIDTH 8",
        4 => b"CMD6 BUS_WIDTH 4",
        _ => b"CMD6 BUS_WIDTH 1",
    }
}
