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

//! A finished image fetch: follow its redirect, or decode what came.

use crate::browser::fetch::types::Fetch;
use crate::browser::http;
use crate::browser::state::State;

/*
 * CDNs answer image hits with 3xx routinely; the hop is chased while the
 * pixels stay keyed to the URL the boxes reference. Anything else goes
 * through ingest, so a failed or empty body is recorded as such and the box
 * keeps its fallback look.
 */
/// Land the image stored under `key`; true when the page has changed.
pub(in crate::browser::fetch) fn land_image(
    state: &mut State,
    job: &Fetch,
    key: &str,
    raw: Option<&[u8]>,
) -> bool {
    let resp = raw.and_then(http::response::parse);
    if let Some(r) = resp.as_ref().filter(|r| matches!(r.status, 301 | 302 | 303 | 307 | 308)) {
        if let Some(loc) = r.location.as_deref() {
            let abs = crate::browser::url::join(&job.url, loc);
            if crate::browser::image::follow_redirect(state, &abs, key, job.hops) {
                return false;
            }
        }
    }
    let body = resp.filter(|r| r.status == 200).map(|r| r.body).unwrap_or_default();
    crate::browser::image::ingest(&mut state.images, key, &body);
    /* A box that waited on this image's natural size lays out again. */
    let sized = state.images.take_natural_dirty();
    if sized && state.box_doc.as_ref().is_some_and(|d| d.awaits(key, state.base.as_ref())) {
        crate::browser::event::relayout(state);
    }
    true
}
