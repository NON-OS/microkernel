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

//! The meter's channels on its screen (B8): each a line to pick, and for the
//! picked one a top-up amount and the two things the wallet may ask: top it
//! up, or close it. Each starts a job in the meter (v1.4.14).

use alloc::format;
use nonos_app_skeleton::PaintBuffer;
use nonos_payment_proto::Account;

use crate::wallet::arm::gate::greyed;
use crate::wallet::etna::parts::fact::fact;
use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::rect::Rect;
use crate::wallet::etna::tokens::GAP;
use crate::wallet::meter::channel::channel_line;
use crate::wallet::nox::nox_text as nox;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::screen::shield::parts::field;
use crate::wallet::state::State;

pub const FIELD_TOPUP: u8 = 2;
pub use crate::wallet::arm::rows::{ROW_CLOSE, ROW_TOPUP};

pub fn channels(state: &State, a: &Account, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let m = &state.meter;
    let mut h = 0;
    for (i, row) in a.rows.iter().enumerate() {
        let label = if m.picked == Some(i as u8) { "channel, picked" } else { "channel" };
        let used = fact(fb, c.x, y + h, c.w, label, &channel_line(row, nox));
        hits::put(Press::Row(i as u8), Rect::new(c.x, y + h, c.w, used));
        h += used;
    }
    h += GAP / 2 + super::claim::row(state, fb, c, y + h + GAP / 2);
    let Some(i) = m.picked.filter(|i| (*i as usize) < a.rows.len()) else {
        return h;
    };
    h += GAP / 2;
    let name = format!("TOP UP CHANNEL {}, WHOLE NOX", i + 1);
    let (at, used) =
        field(fb, c, y + h, &name, &m.topup_text, "for example 50", m.focus == FIELD_TOPUP);
    hits::put(Press::Field(FIELD_TOPUP), at);
    h += used + GAP / 2;
    let idle = m.job.is_none();
    let on = |row| idle && !greyed(state, Press::Row(row));
    let rows =
        [("Top up the picked channel", on(ROW_TOPUP)), ("Close the picked channel", on(ROW_CLOSE))];
    let mut at = [Rect::new(0, 0, 0, 0); 2];
    h += quiet_group(fb, c.x, y + h, c.w, &rows, &mut at);
    for (n, r) in at.iter().enumerate() {
        if rows[n].1 {
            hits::put(Press::Row(ROW_TOPUP + n as u8), *r);
        }
    }
    h + GAP / 2
}
