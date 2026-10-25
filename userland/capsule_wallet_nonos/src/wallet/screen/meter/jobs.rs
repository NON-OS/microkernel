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

//! Top up or close the picked channel: each starts a job in the meter, and
//! the tick asks for its result (v1.4.14). The window never waits.

use nonos_app_skeleton::EventOutcome;
use nonos_payment_proto::{Close, OP_CLOSE, OP_TOPUP};

use crate::wallet::meter::job::{outcome, JobKind, MeterJob, Outcome};
use crate::wallet::meter::topup::topup_request;
use crate::wallet::state::State;

fn picked(state: &State) -> Option<(u64, [u8; 32], [u8; 20])> {
    let a = state.meter.account.as_ref()?;
    let row = a.rows.get(state.meter.picked? as usize)?;
    Some((row.chain_id, row.channel_id, row.reclaim_to))
}

pub(super) fn begin(state: &mut State, kind: JobKind, op: u16, body: &[u8]) -> EventOutcome {
    match crate::wallet::meter::call::start(op, body) {
        Ok(id) => {
            state.meter.job = Some(MeterJob { id, kind, reclaim_to: None });
            state.meter.news = None;
            state.status = b"sent to the meter; it answers when the network has";
        }
        Err(s) => {
            if let Outcome::Failed(why) = outcome(kind, Err(s)) {
                state.failure = Some(why);
            }
        }
    }
    EventOutcome::Repaint
}

pub fn topup(state: &mut State) -> EventOutcome {
    let Some((chain, channel, _)) = picked(state) else {
        return EventOutcome::Idle;
    };
    match topup_request(chain, channel, &state.meter.topup_text) {
        Ok(t) => {
            state.meter.topup_text.clear();
            begin(state, JobKind::TopUp, OP_TOPUP, &t.encode())
        }
        Err(why) => {
            state.failure = Some(why);
            EventOutcome::Repaint
        }
    }
}

pub fn close(state: &mut State) -> EventOutcome {
    let Some((chain_id, channel_id, reclaim_to)) = picked(state) else {
        return EventOutcome::Idle;
    };
    let shown = begin(state, JobKind::Close, OP_CLOSE, &Close { chain_id, channel_id }.encode());
    if let Some(job) = state.meter.job.as_mut().filter(|j| j.kind == JobKind::Close) {
        job.reclaim_to = Some(reclaim_to);
    }
    shown
}
