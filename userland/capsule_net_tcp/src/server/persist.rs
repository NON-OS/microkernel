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
//! The persist timer: probe a closed window that holds data back.

use alloc::vec::Vec;

use crate::state::TABLE;
use crate::tcp::persist::{next_wait, probe_seq, stalled, Persist};
use crate::tcp::{State, FLAG_ACK, MAX_RETX};

/// Probe every connection whose closed window has held its data back for a
/// wait, and drop one whose peer has not answered `MAX_RETX` probes. A
/// peer that answers with a closed window keeps the connection, however
/// long its reader takes.
pub fn scan(now: u64) {
    let mut t = TABLE.lock();
    let mut abort: Vec<u32> = Vec::new();
    for e in t.entries_mut() {
        let sending = matches!(e.tcb.state, State::Established | State::CloseWait);
        if !sending || !stalled(e.tcb.send.wnd, e.tcb.send.una, e.tcb.send.nxt, e.snd_buf.len()) {
            e.persist = Persist::IDLE;
            continue;
        }
        let rto = u64::from(e.rtt.rto_ms());
        if e.persist.due_ms == 0 {
            let wait = next_wait(0, rto);
            e.persist = Persist { due_ms: now.saturating_add(wait), wait_ms: wait, unanswered: 0 };
            continue;
        }
        if now < e.persist.due_ms {
            continue;
        }
        if e.persist.unanswered >= MAX_RETX {
            abort.push(e.handle);
            continue;
        }
        let mut probe = e.tcb;
        probe.send.nxt = probe_seq(e.tcb.send.una);
        probe.recv.wnd = e.rwnd();
        let _ = crate::server::tcp_tx::send(probe, FLAG_ACK, &[]);
        let wait = next_wait(e.persist.wait_ms, rto);
        e.persist = Persist {
            due_ms: now.saturating_add(wait),
            wait_ms: wait,
            unanswered: e.persist.unanswered.saturating_add(1),
        };
    }
    for h in abort {
        t.remove_by_handle(h);
        t.timers.cancel_all(h);
    }
}
