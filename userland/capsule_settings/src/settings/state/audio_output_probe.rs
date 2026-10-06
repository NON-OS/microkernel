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
//! Asks audio.server, each time the Sound page opens, whether this machine
//! can play sound and through what. The server asks the HD Audio driver, which
//! may take up to its own call timeout, so this waits a little longer.

use core::ptr;

use nonos_audio_proto::{output_status_request, read_output_status, HDR_LEN, OUTPUT_REPLY_LEN};
use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use super::audio_output::AudioOutput;

const SERVICE: &[u8] = b"audio.server";
const CALL_TIMEOUT_MS: u64 = 2500;

pub fn probe_audio_output() -> AudioOutput {
    let mut port: u32 = 0;
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port as *mut u32, ptr::null_mut());
    if rc != 0 || port == 0 {
        return AudioOutput::NoService;
    }
    let mut req = [0u8; HDR_LEN];
    let n = output_status_request(&mut req, 1);
    let mut resp = [0u8; OUTPUT_REPLY_LEN];
    let got =
        mk_ipc_call_timeout(port as u64, req.as_ptr(), n, resp.as_mut_ptr(), resp.len(), CALL_TIMEOUT_MS);
    if got <= 0 {
        return AudioOutput::NoService;
    }
    match read_output_status(&resp[..got as usize]) {
        Some((code, flags)) => AudioOutput::Status { code, flags },
        None => AudioOutput::NoService,
    }
}
