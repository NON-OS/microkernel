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

/*
 * Claiming the NOX the Shield pool credited the meter (op 16, v1.4.17): a
 * push the pool could not make waits for the meter alone to claim. The row
 * shows whatever channel is picked, since a claim names no channel; the
 * meter answers ENOENT when nothing waits, which reads as such.
 */

use nonos_app_skeleton::{EventOutcome, PaintBuffer};
use nonos_nox_config::SEPOLIA;
use nonos_payment_proto::OP_CLAIM;

use crate::wallet::arm::gate::greyed;
use crate::wallet::etna::parts::quiet::quiet_group;
use crate::wallet::etna::rect::Rect;
use crate::wallet::meter::job::JobKind;
use crate::wallet::screen::hits::{self, Press};
use crate::wallet::state::State;

pub use crate::wallet::arm::rows::ROW_CLAIM;

pub fn row(state: &State, fb: &mut PaintBuffer, c: Rect, y: u32) -> u32 {
    let idle = state.meter.job.is_none() && !greyed(state, Press::Row(ROW_CLAIM));
    let mut at = [Rect::new(0, 0, 0, 0); 1];
    let h = quiet_group(
        fb,
        c.x,
        y,
        c.w,
        &[("Claim NOX the Shield pool credited the meter", idle)],
        &mut at,
    );
    if idle {
        hits::put(Press::Row(ROW_CLAIM), at[0]);
    }
    h
}

pub fn claim(state: &mut State) -> EventOutcome {
    super::jobs::begin(state, JobKind::Claim, OP_CLAIM, &SEPOLIA.to_le_bytes())
}
