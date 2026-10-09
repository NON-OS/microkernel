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

//! What the volume keys do to the master volume, and the notice that says
//! what came of it. Kept apart from the IPC (sound/volume.rs) so each rule
//! is proven on the host (desktop_proofs volume_tests).
//!
//! The keys reach the shell whatever has focus (the input router's
//! route/shell_keys.rs). A held key repeats as more presses, so Volume Up held
//! keeps stepping; Mute acts once per press, its repeats dropped by the
//! keyboard drivers.

use nonos_app_skeleton::{KEY_MUTE, KEY_VOLUME_DOWN, KEY_VOLUME_UP};
use nonos_audio_proto::{output_short, volume_request, MasterVolume, VOLUME_MAX};

/// One press moves the level this far, out of `VOLUME_MAX`: twenty steps from
/// silence to full, fine enough to land on a comfortable level and few enough
/// to cross in a couple of seconds of a held key.
pub const VOLUME_STEP: u8 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VolumeKey {
    Down,
    Up,
    Mute,
}

/// The volume key `code` names, or None for any other key.
pub fn volume_key(code: u32) -> Option<VolumeKey> {
    match code {
        KEY_VOLUME_DOWN => Some(VolumeKey::Down),
        KEY_VOLUME_UP => Some(VolumeKey::Up),
        KEY_MUTE => Some(VolumeKey::Mute),
        _ => None,
    }
}

/// The volume a press of `key` asks for, from `now`. A step clamps at silence
/// and at full rather than wrapping. A step either way also unmutes, as on
/// every desktop a person has used: pressing Volume Up and hearing nothing
/// reads as a broken key.
pub fn after_key(now: MasterVolume, key: VolumeKey) -> MasterVolume {
    match key {
        VolumeKey::Up => MasterVolume {
            level: now.level.saturating_add(VOLUME_STEP).min(VOLUME_MAX),
            muted: false,
        },
        VolumeKey::Down => MasterVolume {
            level: now.level.min(VOLUME_MAX).saturating_sub(VOLUME_STEP),
            muted: false,
        },
        VolumeKey::Mute => MasterVolume { level: now.level.min(VOLUME_MAX), muted: !now.muted },
    }
}

/// The volume a press of `key` asks for from `now`, and its request to the
/// audio service laid into `out`: the bytes written, zero when `out` is too
/// short to hold one.
pub fn press(
    now: MasterVolume,
    key: VolumeKey,
    request_id: u32,
    out: &mut [u8],
) -> (MasterVolume, usize) {
    let want = after_key(now, key);
    (want, volume_request(out, request_id, want))
}

/// What a volume change came to, as the notice says it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Heard {
    /// The service took it and this is what plays.
    Playing(MasterVolume),
    /// Nothing plays. The audio service's output code says why, or None when
    /// the service itself did not answer.
    NoOutput(Option<u32>),
}

const VOLUME: &[u8] = b"Volume ";
const MUTED: &[u8] = b"Muted";
const NO_OUTPUT: &[u8] = b"No sound output";

/// The notice for `heard`, in `out`: "Volume 45%", "Muted", or "No sound
/// output" with the reason in a few words. Returns the bytes written; `out`
/// holds a toast's worth (`TOAST_TEXT_MAX`), which every line here fits.
pub fn notice(heard: Heard, out: &mut [u8]) -> usize {
    let mut line = Line { out, n: 0 };
    match heard {
        Heard::Playing(v) if v.muted => line.put(MUTED),
        Heard::Playing(v) => {
            line.put(VOLUME);
            line.number(v.level.min(VOLUME_MAX));
            line.put(b"%");
        }
        Heard::NoOutput(None) => line.put(NO_OUTPUT),
        Heard::NoOutput(Some(code)) => {
            line.put(NO_OUTPUT);
            line.put(b": ");
            line.put(output_short(code).as_bytes());
        }
    }
    line.n
}

/// Whether a toast's `text` is a volume notice, which the next one replaces
/// rather than stacks under: a held key would otherwise fill the panel with
/// the levels it passed through.
pub fn is_volume_notice(text: &[u8]) -> bool {
    text.starts_with(VOLUME) || text == MUTED || text.starts_with(NO_OUTPUT)
}

struct Line<'a> {
    out: &'a mut [u8],
    n: usize,
}

impl Line<'_> {
    fn put(&mut self, bytes: &[u8]) {
        let take = bytes.len().min(self.out.len() - self.n);
        self.out[self.n..self.n + take].copy_from_slice(&bytes[..take]);
        self.n += take;
    }

    fn number(&mut self, value: u8) {
        let digits = [b'0' + value / 100, b'0' + value / 10 % 10, b'0' + value % 10];
        let first = if value >= 100 {
            0
        } else if value >= 10 {
            1
        } else {
            2
        };
        self.put(&digits[first..]);
    }
}
