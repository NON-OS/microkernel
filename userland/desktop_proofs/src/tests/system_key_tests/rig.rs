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

//! Volumes and notices as the tests write them.

use crate::shell_system_keys::volume::{notice, Heard};
use crate::toast_state::toast::TOAST_TEXT_MAX;
use nonos_audio_proto::MasterVolume;

pub(super) fn v(level: u8, muted: bool) -> MasterVolume {
    MasterVolume { level, muted }
}

pub(super) fn said(heard: Heard) -> String {
    let mut line = [0u8; TOAST_TEXT_MAX];
    let n = notice(heard, &mut line);
    String::from_utf8(line[..n].to_vec()).unwrap()
}
