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

use nonos_audio_proto::{OUTPUT_READY, VOLUME_MSG_LEN};

use super::level_kept::{in_force, keep};
use super::output::output;
use super::volume_call::call;
use crate::state::volume::{press, Heard, VolumeKey};

// The service answers a volume change itself, without the driver, so a wedged
// service costs a key press this and no more, as a tone does (play.rs).
pub(super) const SET_TIMEOUT_MS: u64 = 100;
pub(super) const REQUEST_ID: u32 = 2;

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
