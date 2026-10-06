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

//! What a finished sub-resource fetch does to the page.

use super::free::free;
use super::respond::{body_ok, dead_kept, response};
use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::pool::Refused;
use crate::browser::fetch::types::Fetch;
use crate::browser::fetch::wire::Wire;
use crate::browser::state::State;

/// Land `job` on the page, and say whether that changed what is drawn.
pub(in crate::browser::fetch) fn land(state: &mut State, w: &mut NetWire, mut job: Fetch) -> bool {
    if dead_kept(&job) && relaunch(state, w, &job).is_ok() {
        w.close(job.handle);
        return false;
    }
    /* A response cut short by a close is no response: an image, sheet or
     * script made of part of one is worse than none. */
    let raw = response(&mut job).filter(|_| !job.truncated);
    free(state, w, &mut job, raw.as_deref());
    if let Some(key) = job.image.take() {
        return super::land_image::land_image(state, &job, &key, raw.as_deref());
    }
    if job.font != 0 {
        let landed = crate::browser::fonts::ingest_font(job.font, body_ok(raw.as_deref()));
        if landed {
            crate::browser::event::relayout(state);
        }
        return landed;
    }
    if job.css {
        crate::browser::fetch::apply_css::apply_css(state, raw.as_deref(), Some(&job.url));
        super::run_held(state);
        return true;
    }
    if job.script {
        state.pool.held.push((job.order, body_ok(raw.as_deref())));
        return super::run_held(state);
    }
    job.js_req && super::land_js::land_js(state, raw.as_deref())
}

/// The same request again, on a new connection, marked as `job` was.
fn relaunch(state: &mut State, w: &mut NetWire, job: &Fetch) -> Result<(), Refused> {
    let proxy = state.proxy.as_ref().map(|p| (p.host.as_str(), p.port));
    let next = state.pool.start_fresh(w, job.url.clone(), proxy)?;
    next.image = job.image.clone();
    next.hops = job.hops;
    next.css = job.css;
    next.font = job.font;
    next.script = job.script;
    next.order = job.order;
    next.js_req = job.js_req;
    Ok(())
}
