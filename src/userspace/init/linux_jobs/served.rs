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

//! The served launches' installs (served_rule.rs), kept for the session.
//! Keyed by a listing the kernel admitted as a capsule's SERVES pairing
//! (app_launch_rule.rs), so it holds at most one entry for each row of
//! `serves::LISTINGS`.

use alloc::collections::BTreeMap;
use alloc::string::String;

use spin::Mutex;

use super::served_rule::{step, Step, Tries};

static TRIES: Mutex<BTreeMap<String, Tries>> = Mutex::new(BTreeMap::new());

/// See `queue::request_served_run`. True only when the core's run was
/// queued: while its install is queued, moving or waiting after a failure,
/// the launch is answered busy, so the window keeps asking until the core
/// is placed and run, rather than looking for a core that is not there.
pub(super) fn request(listing: String, package: String) -> bool {
    let stage = super::status::get(&listing);
    let mut tries = TRIES.lock();
    let kept = tries.get(&listing).copied();
    let (next, keep) = step(stage, kept, crate::time::timestamp_millis());
    let done = match next {
        Step::Run => super::queue::request_run(package),
        Step::Wait => true,
        Step::Install => super::queue::request_install(listing.clone(), String::new()),
    };
    // An install the queue refused was not queued, so nothing is kept of it.
    if done {
        match keep {
            Some(t) => tries.insert(listing, t),
            None => tries.remove(&listing),
        };
    }
    done && next == Step::Run
}
