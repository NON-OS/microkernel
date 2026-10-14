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

//! The spends kept before the last, as a follow reports them.

use alloc::string::String;

use super::reply::{self, SETTLING};
use crate::wallet::state::State;

/* The spends kept before the last, as the service followed them. Each the
 * wallet holds without a number takes the next number it has not seen, in
 * order; one the service holds that this window never saw, as after a
 * restart, is followed all the same; one that landed or went another way
 * is said and let go. */
pub(super) fn followed_earlier(state: &mut State, body: &str) {
    let Some(reported) = reply::kept_all(body) else { return };
    let ui = &mut state.shield_ui;
    let known: alloc::vec::Vec<String> = ui.earlier.iter().filter_map(|s| s.id.clone()).collect();
    let known: alloc::vec::Vec<&str> = known.iter().map(String::as_str).collect();
    let ids: alloc::vec::Vec<&str> = reported.iter().map(|k| k.id).collect();
    let unnamed = ui.earlier.iter().filter(|s| s.id.is_none()).count();
    let (older, newer) = super::kept_names::split(&known, unnamed, &ids);
    let mut newer = newer.into_iter();
    for s in ui.earlier.iter_mut().filter(|s| s.id.is_none()) {
        s.id = newer.next().map(String::from);
    }
    super::follow_older::insert_older(ui, older);
    let mut landed_news = None;
    for s in ui.earlier.iter_mut() {
        let Some(k) = reported.iter().find(|k| Some(k.id) == s.id.as_deref()) else {
            continue;
        };
        let settling = s.state == SETTLING;
        s.minutes = k.minutes;
        s.self_settle = k.self_settle && !settling;
        s.state = String::from(reply::said(k.state, settling));
        if reply::landed(k.state) {
            s.tx = Some(String::from(k.tx));
            landed_news = Some(reply::landed_news("An earlier payment", k.tx));
        }
    }
    /* Landed, or no longer kept by the service: nothing more to follow. */
    ui.earlier.retain(|s| {
        s.tx.is_none() && (s.id.is_none() || reported.iter().any(|k| Some(k.id) == s.id.as_deref()))
    });
    if landed_news.is_some() {
        ui.news = landed_news;
        ui.link.last_sync_ms = 0;
    }
}
