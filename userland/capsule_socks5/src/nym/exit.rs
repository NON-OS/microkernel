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

use core::sync::atomic::{AtomicU32, AtomicU8, Ordering};

use spin::Mutex;

use super::choose::{Named, Record};
use super::watch::Watch;

/// The Nym address of the network requester that opens TCP on our behalf.
///
/// Empty until `exit()` fills it. `set_exit` can set it, but nothing calls
/// that today: `exit()` takes the first exit `discover_exit` finds in the
/// directory, else one of the compiled `BOOTSTRAP_EXITS`, and keeps it here.
/// An exit sees the hosts a client asks for, so that choice is made for you.
static EXIT: Mutex<Option<Exit>> = Mutex::new(None);

/// Position in the directory's exit list. `find_exit` wraps it, so it only
/// ever grows; each rotation moves one node further along.
static INDEX: AtomicU32 = AtomicU32::new(0);

/// Delivery record for the exit in `EXIT`. See the module for the rule it
/// enforces: lookups prove nothing, only delivery does.
static WATCH: Mutex<Watch> = Mutex::new(Watch::new());

/// Which exits went silent on this session and which delivered (`choose`).
static RECORD: Mutex<Record<Exit>> = Mutex::new(Record::new());

/// Exits walked away from this session for silence, saturating.
static ROTATIONS: AtomicU8 = AtomicU8::new(0);

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

pub fn set_exit(exit: Exit) {
    *EXIT.lock() = Some(exit);
    let mut watch = WATCH.lock();
    watch.on_rotate();
    watch.configured = true;
}

/// A send left for the current exit.
pub fn note_sent() {
    WATCH.lock().on_send(nonos_libc::mk_uptime_ms());
}

/// A message came back through the current exit, which proves it.
pub fn note_delivered() {
    WATCH.lock().on_delivered();
    if let Some(exit) = *EXIT.lock() {
        RECORD.lock().proven(exit);
    }
}

/// The current exit answered without payload: alive, not yet proven.
pub fn note_answered() {
    WATCH.lock().on_answered(nonos_libc::mk_uptime_ms());
}

/// Walk to the next exit if the current one has used up its silence budget.
///
/// Returns whether a rotation happened. The caller owns the consequences:
/// the session bound to the old exit is dead weight and has to be reopened,
/// and connections opened through it will never be answered.
pub fn rotate_if_silent() -> bool {
    let mut watch = WATCH.lock();
    if !watch.should_rotate(nonos_libc::mk_uptime_ms()) {
        return false;
    }
    watch.on_rotate();
    drop(watch);
    if let Some(silent) = EXIT.lock().take() {
        RECORD.lock().silent(&silent);
    }
    INDEX.fetch_add(1, Ordering::AcqRel);
    let _ = ROTATIONS.fetch_update(Ordering::AcqRel, Ordering::Acquire, |n| n.checked_add(1));
    true
}

/// Exits walked away from this session for silence.
pub fn rotations() -> u8 {
    ROTATIONS.load(Ordering::Acquire)
}

/// An exit went silent this session, and the one now in use has not
/// delivered yet: the proxy is trying another.
pub fn trying_another() -> bool {
    rotations() > 0 && !WATCH.lock().proven
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
        super::discover::discover_exit(place)
            .or_else(|| super::bootstrap::bootstrap_exit(place as usize))
    };
    let (found, place) = RECORD.lock().pick(index, at)?;
    INDEX.store(place, Ordering::Release);
    *slot = Some(found);
    Some(found)
}
