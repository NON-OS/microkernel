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

//! Whether this machine has anything to play sound on, as the audio
//! service says it, asked once it is settled.

use core::sync::atomic::{AtomicU32, Ordering};

use nonos_audio_proto::{
    output_status_request, read_output_status, HDR_LEN, OUTPUT_NOT_ANSWERING, OUTPUT_REPLY_LEN,
};
use nonos_libc::mk_ipc_call_timeout;

use super::volume::REQUEST_ID;

// The service asks the HD Audio driver, which takes up to its own one second
// call timeout (capsule_audio sink/client.rs) to say it is not answering.
const OUTPUT_TIMEOUT_MS: u64 = 1100;

// The service's output code once it is settled; NOT_LEARNT until then. A
// machine's sound hardware does not change while it runs, so the answer is
// asked for once and not on every key press, which keeps a held key from
// waiting on the driver. "Not answering" is not settled: the driver may be
// still bringing the controller up.
const NOT_LEARNT: u32 = u32::MAX;
static OUTPUT: AtomicU32 = AtomicU32::new(NOT_LEARNT);

/// The service's output code, asked once it is settled; None when the service
/// did not say, in which case the level is shown as set.
pub(super) fn output(port: u32) -> Option<u32> {
    let known = OUTPUT.load(Ordering::Relaxed);
    if known != NOT_LEARNT {
        return Some(known);
    }
    let mut req = [0u8; HDR_LEN];
    let n = output_status_request(&mut req, REQUEST_ID);
    let mut resp = [0u8; OUTPUT_REPLY_LEN];
    let got = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        n,
        resp.as_mut_ptr(),
        resp.len(),
        OUTPUT_TIMEOUT_MS,
    );
    if got <= 0 {
        return None;
    }
    let (code, _) = read_output_status(&resp[..(got as usize).min(resp.len())])?;
    if code != OUTPUT_NOT_ANSWERING {
        OUTPUT.store(code, Ordering::Relaxed);
    }
    Some(code)
}
