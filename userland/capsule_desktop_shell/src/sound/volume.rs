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

//! Asking the audio service to change the master volume for a volume key, and
//! whether this machine has anything to play it on.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use nonos_audio_proto::{
    output_status_request, read_output_status, read_volume_reply, volume_query, MasterVolume,
    E_NODEV, HDR_LEN, OUTPUT_NOT_ANSWERING, OUTPUT_NO_DEVICE, OUTPUT_READY, OUTPUT_REPLY_LEN,
    VOLUME_MSG_LEN, VOLUME_REPLY_LEN,
};
use nonos_libc::mk_ipc_call_timeout;

use crate::state::volume::{press, Heard, VolumeKey};

// The service answers a volume change itself, without the driver, so a wedged
// service costs a key press this and no more, as a tone does (play.rs).
const SET_TIMEOUT_MS: u64 = 100;
// The service asks the HD Audio driver, which takes up to its own one second
// call timeout (capsule_audio sink/client.rs) to say it is not answering.
const OUTPUT_TIMEOUT_MS: u64 = 1100;

const REQUEST_ID: u32 = 2;

// The service's output code once it is settled; NOT_LEARNT until then. A
// machine's sound hardware does not change while it runs, so the answer is
// asked for once and not on every key press, which keeps a held key from
// waiting on the driver. "Not answering" is not settled: the driver may be
// still bringing the controller up.
const NOT_LEARNT: u32 = u32::MAX;
static OUTPUT: AtomicU32 = AtomicU32::new(NOT_LEARNT);

// The volume in force as the service last said it, and whether it has said it
// since the shell started.
static LEVEL: AtomicU32 = AtomicU32::new(MasterVolume::FULL.level as u32);
static MUTED: AtomicBool = AtomicBool::new(false);
static LEARNT: AtomicBool = AtomicBool::new(false);

/// Ask for the volume a press of `key` gives, and say what came of it.
pub fn volume_key(key: VolumeKey) -> Heard {
    let Some(port) = super::audio_port::port() else { return Heard::NoOutput(None) };
    let now = match in_force(port) {
        Ok(now) => now,
        Err(heard) => return heard,
    };
    let mut req = [0u8; VOLUME_MSG_LEN];
    let (_, n) = press(now, key, REQUEST_ID, &mut req);
    let set = match call(port, &req[..n], SET_TIMEOUT_MS) {
        Ok(volume) => volume,
        Err(heard) => return heard,
    };
    keep(set);
    match output(port) {
        Some(code) if code != OUTPUT_READY => Heard::NoOutput(Some(code)),
        _ => Heard::Playing(set),
    }
}

/// The volume to step from: the service's, asked once per shell, so a shell
/// started after the level was turned down does not step from full.
fn in_force(port: u32) -> Result<MasterVolume, Heard> {
    if LEARNT.load(Ordering::Relaxed) {
        let level = LEVEL.load(Ordering::Relaxed) as u8;
        return Ok(MasterVolume { level, muted: MUTED.load(Ordering::Relaxed) });
    }
    let mut req = [0u8; HDR_LEN];
    let n = volume_query(&mut req, REQUEST_ID);
    let now = call(port, &req[..n], SET_TIMEOUT_MS)?;
    keep(now);
    Ok(now)
}

fn keep(volume: MasterVolume) {
    LEVEL.store(u32::from(volume.level), Ordering::Relaxed);
    MUTED.store(volume.muted, Ordering::Relaxed);
    LEARNT.store(true, Ordering::Relaxed);
}

/// One `OP_SET_VOLUME` call: the volume in force after it, or what to say
/// instead. A service with no driver refuses with E_NODEV, which is a machine
/// with no sound hardware; any other failure is a service that did not
/// answer.
fn call(port: u32, req: &[u8], timeout_ms: u64) -> Result<MasterVolume, Heard> {
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

/// The service's output code, asked once it is settled; None when the service
/// did not say, in which case the level is shown as set.
fn output(port: u32) -> Option<u32> {
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
