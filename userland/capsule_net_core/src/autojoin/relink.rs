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

//! A rejoin pass and the watch of a join it hands the driver. Each pass asks
//! the link first: a person who left the network in the meantime ends the
//! rejoin, and the count of joins the driver has finished is noted, so that
//! while the join runs on in the driver only a result newer than it is taken.

use spin::Mutex;

use nonos_wifi_client::{find, Link, LinkEnded};

use super::attempt::attempt;
use super::link::up_on;
use super::machine::{Machine, Step, CALL_MS};
use super::tick::say;

/// The joins the driver had finished when the pass began.
static JOINS_BEFORE: Mutex<Option<u8>> = Mutex::new(None);

fn link() -> Option<Link> {
    find().and_then(|d| d.link_within(CALL_MS))
}

/// One rejoin pass for the network that was up.
pub(super) fn rejoin_pass() -> Step {
    let seen = link();
    if seen.is_some_and(|l| l.ended == LinkEnded::Left) {
        return Step::Done;
    }
    *JOINS_BEFORE.lock() = seen.and_then(|l| l.last_join).map(|j| j.seq);
    let (ssid, len) = up_on();
    attempt(0, Some(&ssid[..len]))
}

/// Look at a running join's link; a rejoin also takes a newer result.
pub(super) fn watch(m: &mut Machine, now: i64) {
    let seen = link();
    let before = *JOINS_BEFORE.lock();
    let fresh = seen.and_then(|l| l.last_join).filter(|j| Some(j.seq) != before);
    let ended = match fresh {
        Some(j) if m.rejoining() && !seen.is_some_and(|l| l.associated) => {
            m.join_ended(now, j.code)
        }
        _ => m.after_watch(now, seen.map(|l| l.associated)),
    };
    match ended {
        Some((_, true)) => say(b"[NET-CORE] joined a saved Wi-Fi network\n"),
        Some((_, false)) => say(b"[NET-CORE] saved Wi-Fi network not joined in time\n"),
        None => {}
    }
}
