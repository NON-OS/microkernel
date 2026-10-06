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

//! Which files in /audio the library lists. Only the formats the decoder
//! opens (decode/sniff.rs: MP3 and WAV) are listed, so the library never
//! shows a track that fails to play the moment it is picked.

const PLAYABLE: [&str; 2] = ["mp3", "wav"];

pub fn is_playable(path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    match name.rfind('.') {
        Some(i) if i > 0 => {
            let ext = &name[i + 1..];
            PLAYABLE.iter().any(|p| ext.eq_ignore_ascii_case(p))
        }
        _ => false,
    }
}
