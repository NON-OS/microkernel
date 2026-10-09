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

/*
 * Following the spend proved last until it lands. The service looks at the
 * chain at most once a minute, republishes through the landers when one
 * dropped it, and after 30 minutes offers the owner a settlement from their
 * own account. Inputs count as spent only once it has landed.
 */

use alloc::string::String;

use nonos_libc::mk_uptime_ms;
use shield_wire::*;

use super::reply;
use crate::wallet::state::shield_log::Kind;
use crate::wallet::state::shield_ui::Spend;
use crate::wallet::state::State;

const EVERY_MS: i64 = 60_000;
const NO_STATE: &str = "The shield service's follow did not say where the payment is. It is \
     asked again in a minute.";
/* A spend the owner settled from their own account, followed until it lands. */
pub use super::reply::SETTLING;
/* The pool read again while Shield is open, for receipts and maturity. */
const SYNC_EVERY_MS: i64 = 180_000;

/* Whether the pool was never read for this open store, or not for a sync
 * interval: opening Shield again within it reads nothing new. */
pub fn sync_stale(ui: &crate::wallet::state::shield_ui::ShieldUi, now: i64) -> bool {
    ui.last_sync_ms == 0 || now - ui.last_sync_ms >= SYNC_EVERY_MS
}

/* Whether any spend is not yet in a block, however it is being landed. */
fn following(state: &State) -> bool {
    let ui = &state.shield_ui;
    ui.spend.as_ref().is_some_and(|s| s.tx.is_none()) || !ui.earlier.is_empty()
}

/* Start whatever background read is due. True when one started. */
pub fn due(state: &mut State) -> bool {
    let ui = &state.shield_ui;
    if !ui.opened || ui.waiting.is_some() {
        return false;
    }
    let now = mk_uptime_ms();
    if following(state) && now - ui.last_follow_ms >= EVERY_MS {
        state.shield_ui.last_follow_ms = now;
        return super::job::start(state, OP_FOLLOW, &[]);
    }
    if sync_stale(ui, now) {
        state.shield_ui.last_sync_ms = now;
        return super::job::start(state, OP_SYNC, &[]);
    }
    false
}

pub fn followed(state: &mut State, body: &str) {
    followed_earlier(state, body);
    let ui = &mut state.shield_ui;
    let Some(spend) = ui.spend.as_mut() else { return };
    let Some(r) = reply::last(body) else {
        ui.failure = Some(String::from(NO_STATE));
        return;
    };
    let settling = spend.state == SETTLING;
    spend.self_settle = r.self_settle && !settling;
    spend.minutes = r.minutes.unwrap_or(spend.minutes);
    spend.refusal = r.refusal.map(String::from);
    spend.published |= r.published;
    spend.state = String::from(reply::said(r.said, settling));
    if !reply::landed(r.said) {
        return;
    }
    spend.tx = Some(String::from(r.tx));
    ui.news = Some(reply::landed_news("The payment", r.tx));
    ui.last_sync_ms = 0;
}

/* The spends kept before the last, as the service followed them. Each the
 * wallet holds without a number takes the next number it has not seen, in
 * order; one the service holds that this window never saw, as after a
 * restart, is followed all the same; one that landed or went another way
 * is said and let go. */
fn followed_earlier(state: &mut State, body: &str) {
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
    for (at, id) in older.into_iter().enumerate() {
        ui.earlier.insert(
            at,
            Spend {
                id: Some(String::from(id)),
                kind: Kind::Send,
                asset: 0,
                amount: String::new(),
                published: true,
                state: String::new(),
                minutes: 0,
                tx: None,
                self_settle: false,
                refusal: None,
                weakened: alloc::vec::Vec::new(),
            },
        );
    }
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
        ui.last_sync_ms = 0;
    }
}
