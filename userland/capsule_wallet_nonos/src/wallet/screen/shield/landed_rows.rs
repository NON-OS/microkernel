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
//! The landed rows painted, and a press on one (v1.4.56).

use alloc::vec::Vec;

use nonos_app_skeleton::{EventOutcome, PaintBuffer};

use super::landed_move::{landed, said, ROW_MOVE};
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::order::fees::exact;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::shield::fresh_move_act::{act, label_of, moves, Act};
use crate::wallet::shield::fresh_move_job as job;
use crate::wallet::shield::landed_held::line;
use crate::wallet::state::State;

pub fn paint(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let (mut h, now) = (0, nonos_tls::rtc_now());
    for (i, (address, held)) in (0u8..).zip(landed(state)) {
        h += fact(fb, c.x, y + h, c.w, "landed", &line(&address, held, exact)) + GAP / 2;
        let m = state.meter.landed.moving.as_ref().filter(|m| m.address == address);
        for s in m.map(said).unwrap_or_default() {
            h += fact(fb, c.x, y + h, c.w, "move", &s) + GAP / 2;
        }
        if let Some((eth, _)) = moves(held) {
            let m = m.filter(|m| m.eth == eth);
            let mut at = [Rect::default(); 1];
            let label = label_of(act(m, now), eth);
            h += quiet_group(fb, c.x, y + h, c.w, &Vec::from([(label, true)]), &mut at);
            hits::put(Press::Row(ROW_MOVE + i), at[0]);
        }
    }
    h
}

/// A press on landed row `row`: its button as it reads now.
pub fn click(state: &mut State, row: u8) -> EventOutcome {
    let Some((address, held)) = landed(state).get(usize::from(row.wrapping_sub(ROW_MOVE))).copied()
    else {
        return EventOutcome::Idle;
    };
    let Some((eth, amount)) = moves(held) else { return EventOutcome::Idle };
    let m = state.meter.landed.moving.as_ref().filter(|m| m.address == address && m.eth == eth);
    match act(m, nonos_tls::rtc_now()) {
        Act::Move | Act::GasWaiting => job::start(state, address, amount, eth),
        Act::Confirm { .. } => job::confirm(state),
        Act::PayGas => return job::pay_gas(state),
    }
    EventOutcome::Repaint
}
