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

//! One `OP_SET_VOLUME` call to the audio service.

use nonos_audio_proto::{
    read_volume_reply, MasterVolume, E_NODEV, OUTPUT_NO_DEVICE, VOLUME_REPLY_LEN,
};
use nonos_libc::mk_ipc_call_timeout;

use crate::state::volume::Heard;

/// One `OP_SET_VOLUME` call: the volume in force after it, or what to say
/// instead. A service with no driver refuses with E_NODEV, which is a machine
/// with no sound hardware; any other failure is a service that did not
/// answer.
pub(super) fn call(port: u32, req: &[u8], timeout_ms: u64) -> Result<MasterVolume, Heard> {
    let mut resp = [0u8; VOLUME_REPLY_LEN];
    let got = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        req.len(),
        resp.as_mut_ptr(),
        resp.len(),
        timeout_ms,
    );
    if got <= 0 {
        super::audio_port::forget();
        return Err(Heard::NoOutput(None));
    }
    match read_volume_reply(&resp[..(got as usize).min(resp.len())]) {
        Ok(volume) => Ok(volume),
        Err(E_NODEV) => Err(Heard::NoOutput(Some(OUTPUT_NO_DEVICE))),
        Err(_) => Err(Heard::NoOutput(None)),
    }
}
