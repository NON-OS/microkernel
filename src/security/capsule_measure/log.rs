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

//! The kernel's own PCR 14 events, one for each distinct capsule admitted
//! this boot; a capsule spawned again adds nothing. The firmware's log ends
//! at ExitBootServices, so the kernel keeps its events here, and the
//! document carries them after the firmware's. An extend and its event are
//! taken under one lock, and so is a document's copy of the log with its
//! quote, so the PCR and the log never drift apart.
//!
//! A failed extend is sticky: the log no longer describes PCR 14, so every
//! later document is refused; the boot is not stopped, as `kernel_gate` does
//! not stop it for PCR 9. Only a machine with no TPM skips the extend.

use alloc::vec::Vec;
use spin::Mutex;

use super::event::BODY_LEN;
use crate::smp::lock_responsive;

pub(super) struct Log {
    pub(super) events: Vec<u8>,
    pub(super) bodies: Vec<[u8; BODY_LEN]>,
    pub(super) broken: bool,
}

pub(super) static LOG: Mutex<Log> =
    Mutex::new(Log { events: Vec::new(), bodies: Vec::new(), broken: false });

pub(super) fn broke(log: &mut Log, why: &[u8]) {
    log.broken = true;
    crate::sys::serial::print(why);
    crate::sys::serial::println(b"; attestation refused until reboot");
}

/// Runs `f` over the kernel's events, holding the lock no extend can pass;
/// `None` once an extend has failed.
pub fn with_events<T>(f: impl FnOnce(&[u8]) -> T) -> Option<T> {
    let log = lock_responsive(&LOG);
    match log.broken {
        true => None,
        false => Some(f(&log.events)),
    }
}
