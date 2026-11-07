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

//! A running session at each tick: revocation of either grant, the
//! escalation's window, the session's end and the idle timeout.

use super::Server;
use crate::{Attestation, EndReason, EscEnd, Event, HostSigner, Rand, Roles, Why};

impl<A: Attestation, S: HostSigner, R: Roles> Server<A, S, R> {
    pub(super) fn session_tick(&mut self, now_ms: u64, rand: Rand<'_>) {
        let Some(live) = self.live.as_mut() else { return };
        let base = live.login.grant_id;
        let esc_id = live.active.as_ref().map(|esc| esc.grant_id);
        let held = self
            .roles
            .with(|_, list| (list.contains(&base), esc_id.is_some_and(|id| list.contains(&id))));
        // A host that can no longer vouch for its list fails closed: the
        // session and its escalation end as if both grants were revoked.
        let (revoked, esc_revoked) = match held {
            Ok(r) => r,
            Err(e) => {
                if live.stopping.is_none() {
                    self.events.push(Event::Refused(Why::Policy(e)));
                }
                (true, true)
            }
        };
        let esc_end = live.active.as_ref().and_then(|esc| match esc_revoked {
            true => Some(EscEnd::Revoked),
            false => (now_ms >= esc.ends_ms).then_some(EscEnd::Expired),
        });
        let idle_ms = u64::from(self.cfg.idle_s) * 1000;
        let stop = if revoked {
            Some(EndReason::Host)
        } else if now_ms >= live.login.ends_ms {
            Some(EndReason::Expired)
        } else {
            (now_ms.saturating_sub(live.last_input_ms) >= idle_ms).then_some(EndReason::Idle)
        };
        if let Some(why) = esc_end {
            if let Some(esc) = live.active.take() {
                let _ = self.esc_ended(&esc.hash, why, now_ms, rand);
            }
        }
        if let Some(why) = stop {
            self.stop(why);
        }
    }
}
