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

//! The exit in use: the network requester the proxy routes through.

use core::sync::atomic::{AtomicU32, Ordering};

use spin::Mutex;

use super::super::choose::Named;
use super::record::RECORD;

/// The Nym address of the network requester that opens TCP on our behalf.
///
/// Empty until `exit()` fills it: `exit()` takes the first exit
/// `discover_exit` finds in the directory, else one of the compiled
/// `BOOTSTRAP_EXITS`, and keeps it here. An exit sees the hosts a client
/// asks for, so that choice is made for you.
pub(super) static EXIT: Mutex<Option<Exit>> = Mutex::new(None);

/// Position in the directory's exit list. `find_exit` wraps it, so it only
/// ever grows; each rotation moves one node further along.
pub(super) static INDEX: AtomicU32 = AtomicU32::new(0);

/// A network requester: 32-byte identity, 32-byte encryption key, and the
/// identity of the gateway it sits behind.
#[derive(Clone, Copy)]
pub struct Exit {
    pub identity: [u8; 32],
    pub encryption: [u8; 32],
    pub gateway: [u8; 32],
}

impl Named for Exit {
    fn id(&self) -> [u8; 32] {
        self.identity
    }
}

/// The exit to route through.
///
/// A choice made here wins. Otherwise one is taken from the directory, and
/// only if the network cannot be asked at all does the compiled list stand
/// in: that list ages, and an operator who stops running a requester leaves
/// every client that shipped with it unable to reach anything.
pub fn exit() -> Option<Exit> {
    let mut slot = EXIT.lock();
    if let Some(configured) = *slot {
        return Some(configured);
    }
    let index = INDEX.load(Ordering::Acquire);
    /* The next exit not seen silent this session, else one that delivered,
     * else the one silent longest ago (`choose`). */
    let at = |place: u32| {
        super::super::discover::discover_exit(place)
            .or_else(|| super::super::bootstrap::bootstrap_exit(place as usize))
    };
    let (found, place) = RECORD.lock().pick(index, at)?;
    INDEX.store(place, Ordering::Release);
    *slot = Some(found);
    Some(found)
}
