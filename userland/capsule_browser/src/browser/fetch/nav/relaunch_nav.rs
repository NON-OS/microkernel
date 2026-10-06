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

//! A navigation whose kept connection had been closed, sent again.

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::open::open;
use crate::browser::fetch::progress;
use crate::browser::fetch::types::Fetch;
use crate::browser::fetch::wire::Wire;
use crate::browser::state::State;

/// Send `job`'s request again on a new connection; false if none opens,
/// and the navigation then fails as it stands.
pub(in crate::browser::fetch) fn relaunch_nav(
    state: &mut State,
    w: &mut NetWire,
    job: &mut Fetch,
) -> bool {
    let proxy = state.proxy.as_ref().map(|p| (p.host.as_str(), p.port));
    let Ok(mut next) = open(w, job.url.clone(), proxy) else { return false };
    w.close(job.handle);
    next.suppress = job.suppress;
    next.post = job.post.take();
    next.silent_base = job.silent_base;
    state.status = progress::status(&next);
    state.fetch = Some(next);
    true
}
