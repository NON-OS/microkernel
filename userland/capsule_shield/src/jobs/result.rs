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

//! What the last job answered, as the RESULT reply.

use shield_wire::Values;

use super::phase::phase;
use super::{State, SLOT};

/// What the last job answered, as reply values.
pub fn result() -> Values {
    let mut v = Values::new();
    let Ok(slot) = SLOT.lock() else {
        v.put("state", "none");
        return v;
    };
    match slot.as_ref() {
        None => {
            v.put("state", "none");
        }
        Some(job) => {
            v.put("job", job.name);
            match &job.state {
                State::Running => {
                    v.put("state", "running");
                    phase(&mut v, job.name);
                }
                State::Failed(why) => {
                    v.put("state", "failed").put("why", why);
                }
                State::Done(text) => return crate::reply::done(job.name, text),
            }
        }
    }
    v
}
