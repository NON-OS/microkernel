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

//! The last bring-up check: the card is ready in transfer state.

use super::super::super::env::{Clock, Log, Mmio};
use super::super::super::error::{EmmcError, EmmcResult};
use super::super::super::sdhci::Host;
use super::super::super::text::Line;
use super::super::card::Card;
use super::super::cmds::RCA;
use super::super::ops::{say_failed, status};
use super::super::r1::ready;

/// Read the card's status once more: it must be ready for data in transfer
/// state. Logs the card as brought up.
pub(super) fn check_ready<M: Mmio, C: Clock, L: Log>(
    h: &mut Host<M, C, L>,
    card: &Card,
) -> EmmcResult<()> {
    let s = match status(h, RCA) {
        Ok(s) => s,
        Err(e) => {
            say_failed(h, b"CMD13 SEND_STATUS", e);
            return Err(e);
        }
    };
    if !ready(s) {
        h.say(Line::new().s(b"card not ready after bring-up (status ").hex(s as u64).s(b")"));
        return Err(EmmcError::BadState(s));
    }
    h.say(
        Line::new()
            .s(b"ready: ")
            .dec(card.sectors)
            .s(b" sectors (")
            .dec(card.sectors / 2048)
            .s(b" MiB), ")
            .dec(card.width as u64)
            .s(b"-bit, ")
            .s(if card.hs { b"high speed " as &[u8] } else { b"legacy " })
            .dec((card.hz / 1000) as u64)
            .s(b" kHz"),
    );
    Ok(())
}
