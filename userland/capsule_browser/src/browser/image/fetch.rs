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

use alloc::string::String;

use crate::browser::fetch::{start_image as start, NetWire, Refused};
use crate::browser::state::State;

const MAX_IMG_REDIRECTS: u8 = 4;

/*
 * Images take whatever room the pool has left after stylesheets, fonts and
 * scripts, several at a time. Redirect hops go first, since their image was
 * asked for earlier. A host at its limit holds back only its own images.
 * data: sources carry their bytes inline and are decoded without a socket.
 */
/// Start every image fetch there is room for; true if any image landed.
pub fn pump(state: &mut State, w: &mut NetWire) -> bool {
    let mut landed = false;
    while let Some((target, key, hops)) = state.pool.redirects.first().cloned() {
        match start(state, w, &target, &key, hops) {
            Err(Refused::Busy) => break,
            Err(Refused::Failed(_)) => state.images.set_failed(&key),
            Ok(()) => {}
        }
        state.pool.redirects.remove(0);
    }
    let mut at = 0;
    while at < state.image_queue.len() {
        let target = state.image_queue[at].clone();
        if target.starts_with("data:") {
            match super::data_uri::data_uri_bytes(&target) {
                Some(bytes) => super::ingest(&mut state.images, &target, &bytes),
                None => state.images.set_failed(&target),
            }
            landed = true;
        } else if state.images.ready(&target).is_none() {
            match start(state, w, &target, &target, 0) {
                Err(Refused::Busy) => {
                    at += 1;
                    continue;
                }
                Err(Refused::Failed(_)) => state.images.set_failed(&target),
                Ok(()) => {}
            }
        }
        state.image_queue.remove(at);
    }
    landed
}

/// Queue the hop a 3xx pointed an image at, keyed to the URL the boxes use.
/// False once the hop budget is spent, so the caller records a failure.
pub fn follow_redirect(state: &mut State, location: &str, key: &str, hops: u8) -> bool {
    if hops >= MAX_IMG_REDIRECTS {
        return false;
    }
    state.pool.redirects.push((String::from(location), String::from(key), hops + 1));
    true
}
