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

//! Asking the audio service for one tone.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_audio_proto::{tone_request, TONE_MSG_LEN};
use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

const SERVICE: &[u8] = b"audio.server";

// The shell draws the desktop. A wedged audio service costs it this and no more.
const TIMEOUT_MS: u64 = 100;

const REQUEST_ID: u32 = 1;

static PORT: AtomicU32 = AtomicU32::new(0);

pub(super) fn play(hz: u32, ms: u32, gain: u16) {
    let mut port = PORT.load(Ordering::Relaxed);
    if port == 0 {
        let mut found = 0u32;
        let mut pid = 0u32;
        let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut found, &mut pid);
        if rc < 0 || found == 0 {
            return;
        }
        port = found;
        PORT.store(port, Ordering::Relaxed);
    }
    let mut req = [0u8; TONE_MSG_LEN];
    let n = tone_request(&mut req, REQUEST_ID, hz, ms, gain);
    if n == 0 {
        return;
    }
    let mut resp = [0u8; 32];
    let rc = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        n,
        resp.as_mut_ptr(),
        resp.len(),
        TIMEOUT_MS,
    );
    if rc < 0 {
        // The service may have restarted on a new port; look it up again next time.
        PORT.store(0, Ordering::Relaxed);
    }
}
