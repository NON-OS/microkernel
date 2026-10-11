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

//! The switch to High Speed timing, and the fall back to legacy timing.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::Host;
use super::super::super::text::Line;
use super::super::card::Card;
use super::super::ext_csd::HS_TIMING;
use super::super::ops::{recover, say_failed, status, switch_send, switch_wait};

/// HS_TIMING = 1, then High Speed Enable on the host, then the card's
/// status read at the new timing, then the clock up. A refusal leaves the
/// card at legacy timing; anything that leaves the card unreachable fails.
pub(super) fn high_speed<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &mut Card,
    hz: u32,
) -> EmmcResult<()> {
    if let Err(e) = switch_send(h, card, HS_TIMING, 1) {
        say_failed(h, b"CMD6 HS_TIMING", e);
        return settle_legacy(h, card);
    }
    h.set_timing(true, h.sd_hz)?;
    if let Err(e) = switch_wait(h, card.rca, HS_TIMING, card.switch_ms(HS_TIMING)) {
        say_failed(h, b"HS_TIMING status", e);
        h.set_timing(false, h.sd_hz)?;
        return settle_legacy(h, card);
    }
    card.hz = h.set_clock(hz)?;
    card.hs = true;
    h.say(Line::new().s(b"high speed at ").dec((card.hz / 1000) as u64).s(b" kHz"));
    Ok(())
}

/// After a refused or failed HS switch: the card must still answer in
/// transfer state, or bring-up fails.
fn settle_legacy<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &mut Card,
) -> EmmcResult<()> {
    card.hs = false;
    if !recover(h, card.rca) {
        return Err(EmmcError::BadState(status(h, card.rca).unwrap_or(0)));
    }
    h.say(Line::new().s(b"staying at legacy timing"));
    Ok(())
}
