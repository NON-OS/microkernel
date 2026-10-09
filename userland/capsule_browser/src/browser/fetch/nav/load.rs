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

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::open::open;
use crate::browser::fetch::reuse::reuse;
use crate::browser::fetch::wire::Wire;
use crate::browser::net::mixnet::{silent_now, Network, Way};
use crate::browser::state::State;
use crate::browser::url;

/*
 * A navigation opens its socket and starts connecting here, and returns: the
 * connect is polled by the ticks that follow while the window stays drawn
 * and answers input, and "Connecting to" shows from the first of them. The
 * page on screen stays until the new response commits. A GET to a host with
 * a kept connection goes out on it, in this call.
 */
pub fn load(state: &mut State, target: &str) -> Result<(), &'static str> {
    if target.starts_with("about:") {
        super::teardown::teardown(state, &mut NetWire(state.sockets_port));
    }
    if crate::browser::fetch::about_page::about_page(state, target) {
        return Ok(());
    }
    let url = url::parse(target).ok_or("bad url")?;
    let moved = super::services::services(state, &url.host)?;
    let mut w = NetWire(state.sockets_port);
    if let Some(old) = state.fetch.take() {
        w.close(old.handle);
    }
    let way = w.way(&url.host);
    /*
     * What the old page queued must not start under this page's network:
     * an onion page's images would otherwise go direct once the reader left
     * it for a page on a direct network.
     */
    if moved {
        crate::browser::fetch::cancel_all(state);
    }
    let post = core::mem::take(&mut state.pending_post);
    let proxy = state.proxy.as_ref().map(|p| (p.host.as_str(), p.port));
    let reusable = post.is_none() && (proxy.is_none() || way.proxied());
    let kept = if reusable { state.pool.take_idle(&mut w, &url) } else { None };
    let mut f = match kept.and_then(|idle| reuse(&mut w, idle, url.clone())) {
        Some(f) => f,
        None => {
            /* The navigation comes before what the page being left still
             * fetches, and before connections kept for later. */
            if way.proxied() {
                state.pool.free_stream(&mut w, way);
            }
            open(&mut w, url, proxy)?
        }
    };
    if let Way::Proxy { net: Network::Nym, port, .. } = f.way {
        f.silent_base = silent_now(port);
    }
    f.suppress = core::mem::take(&mut state.suppress_history_push);
    f.post = post;
    state.status = crate::browser::fetch::progress::status(&f);
    state.fetch = Some(f);
    Ok(())
}
