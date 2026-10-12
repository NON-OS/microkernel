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

//! One look's listening: a bonded peer is taken when heard; a new one is
//! taken at once while no bond is kept, else once it has waited HOLD_MS.

use super::env::Env;
use super::find::{Peer, SCAN_MS};
use super::heard::{heard_in, Heard};
use nonos_bt_hci::{HciError, Host, Transport};

/// How long a new advertiser waits, from when it is first heard, while
/// bonds are kept: many advertising intervals of a device that advertises
/// every 100 ms or faster, and little delay to a first pairing.
pub const HOLD_MS: u64 = 2_000;

pub(super) fn listen<T: Transport, E: Env>(
    host: &mut Host<T>,
    env: &mut E,
) -> Result<Option<Peer>, HciError> {
    let until = env.now_ms().saturating_add(SCAN_MS);
    let hold = env.any_bonds();
    let mut waiting: Option<(Peer, u64)> = None;
    loop {
        let now = env.now_ms();
        match waiting {
            Some((peer, at)) if now >= at => return Ok(Some(peer)),
            _ if now >= until => return Ok(waiting.map(|(peer, _)| peer)),
            _ => {}
        }
        let Some(event) = host.next_event()? else {
            host.transport().pause();
            continue;
        };
        match heard_in(&event, env)? {
            Some(Heard::Bonded(peer)) => return Ok(Some(peer)),
            Some(Heard::New(peer)) if !hold => return Ok(Some(peer)),
            Some(Heard::New(peer)) if waiting.is_none() => {
                waiting = Some((peer, now.saturating_add(HOLD_MS)));
            }
            _ => {}
        }
    }
}
