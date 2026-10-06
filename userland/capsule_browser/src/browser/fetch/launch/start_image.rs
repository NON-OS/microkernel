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

//! Starting one image fetch in the pool.

use alloc::string::String;

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::pool::Refused;
use crate::browser::state::State;
use crate::browser::url;

/// Start one image fetch, keyed to `key`, `hops` redirects in.
pub fn start_image(
    state: &mut State,
    w: &mut NetWire,
    target: &str,
    key: &str,
    hops: u8,
) -> Result<(), Refused> {
    let proxy = state.proxy.as_ref().map(|p| (p.host.as_str(), p.port));
    let started = match url::parse(target) {
        Some(u) => state.pool.start(w, u, proxy),
        None => Err(Refused::Failed("bad url")),
    };
    started.map(|f| {
        f.image = Some(String::from(key));
        f.hops = hops;
    })
}
