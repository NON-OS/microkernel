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

//! The system's master volume, the one the keyboard's volume keys step: the
//! player's slider and mute set it, and show it, so the window and the keys
//! never disagree about how loud the machine is. Music no longer keeps a
//! volume of its own on top, which turned a sound down twice.

use nonos_audio_proto::{
    read_volume_reply, volume_query, volume_request, MasterVolume, HDR_LEN, VOLUME_MSG_LEN,
    VOLUME_REPLY_LEN,
};
use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

const SERVICE_NAME: &[u8] = b"audio.server";
const REQUEST_ID: u32 = 3;
/// The service answers a volume call itself, without its driver, so a call
/// that takes longer than this is a service that is not there.
const TIMEOUT_MS: u64 = 100;

/// The volume in force, or None when the audio service did not say.
pub fn master() -> Option<MasterVolume> {
    let mut req = [0u8; HDR_LEN];
    let n = volume_query(&mut req, REQUEST_ID);
    call(&req[..n])
}

/// Set the volume; the volume in force after, or None when the service did
/// not take it.
pub fn set_master(volume: MasterVolume) -> Option<MasterVolume> {
    let mut req = [0u8; VOLUME_MSG_LEN];
    let n = volume_request(&mut req, REQUEST_ID, volume);
    if n == 0 {
        return None;
    }
    call(&req[..n])
}

fn call(req: &[u8]) -> Option<MasterVolume> {
    let mut port = 0u32;
    let mut pid = 0u32;
    let rc = mk_service_lookup(SERVICE_NAME.as_ptr(), SERVICE_NAME.len(), &mut port, &mut pid);
    if rc < 0 || port == 0 {
        return None;
    }
    let mut resp = [0u8; VOLUME_REPLY_LEN];
    let got = mk_ipc_call_timeout(
        port as u64,
        req.as_ptr(),
        req.len(),
        resp.as_mut_ptr(),
        resp.len(),
        TIMEOUT_MS,
    );
    if got <= 0 {
        return None;
    }
    read_volume_reply(&resp[..(got as usize).min(resp.len())]).ok()
}
