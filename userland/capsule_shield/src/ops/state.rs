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

//! The answer to a state request, from what the worker read last.

use shield_wire::Values;

use crate::jobs;

use super::seen::SEEN;
use super::state_history::history;
use super::Answer;

pub fn state() -> Answer {
    let mut v = Values::new();
    /* The running job goes first: it is what the wallet waits on, and
     * nothing after it can push it out of the reply. */
    if let Some(job) = jobs::running() {
        v.put("job", job);
    }
    let seen = SEEN.lock().map_err(|_| String::from("the shield is busy"))?;
    match seen.as_ref() {
        None => {
            v.put("unlocked", "0");
        }
        Some(s) => {
            v.put("unlocked", "1").put("account", &s.account);
            v.put("kept", if s.live { "memory" } else { "volume" });
            if let Some(a) = &s.address {
                v.put("address", a);
            }
            /* Only what was read is answered: a store that would not give
             * them says why, and the wallet keeps what it showed before. */
            match &s.balances {
                Ok(all) => {
                    v.put("balances", "1");
                    for b in all {
                        v.put("balance", b);
                    }
                }
                Err(why) => {
                    v.put("balances_why", why);
                }
            }
            history(&mut v, s);
        }
    }
    drop(seen);
    Ok(v)
}
