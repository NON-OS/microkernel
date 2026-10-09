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

//! What the player says when it cannot play: the line the transport bar shows
//! in red under the track title, in place of the artist.

use nonos_audio_proto::is_output_message;

use crate::track_limit::TOO_LARGE;

/// The feed fault of the sink that stands in when `audio.server` was not found
/// at start: every feed fails with it, so play stops at once and says why.
pub const NO_SERVICE: &str = "No audio output: the audio service is not running";

/// The transport bar's line for a feed fault, or for a stream the audio
/// service would not open.
pub fn output_trouble(fault: &'static str) -> &'static str {
    match fault {
        NO_SERVICE => NO_SERVICE,
        // The audio service named why this machine cannot play: say that.
        _ if is_output_message(fault) => fault,
        "audio.server feed: no reply" | "audio.server open: no reply" => {
            "No audio output: the audio service stopped answering"
        }
        "audio.server feed rejected" | "audio.server open rejected" => {
            "No audio output: the audio service refused the sound"
        }
        _ => "No audio output",
    }
}

/// The transport bar's line for a track that would not load: the file could
/// not be read, is not a format the player decodes, or the audio service
/// would not open a stream for it.
pub fn track_trouble(err: &'static str) -> &'static str {
    match err {
        "unknown audio format" => "Cannot play: only MP3 and WAV files can be decoded",
        TOO_LARGE => "Cannot play: the file is larger than 32 MiB",
        "vfs ipc failed" => "Cannot play: the file store did not answer",
        "vfs open failed" | "vfs read failed" | "vfs path invalid" => {
            "Cannot play: the file could not be read"
        }
        _ if err.starts_with("audio.server") || is_output_message(err) => output_trouble(err),
        _ => "Cannot play: the file is damaged or not audio",
    }
}

/// The line for a library with nothing to play: the music folder could not be listed
/// (`scan_error`), or it holds no track.
pub fn library_trouble(scan_error: Option<&'static str>, tracks: usize) -> Option<&'static str> {
    match scan_error {
        Some("vfs ipc failed") => Some("No music: the file store did not answer"),
        Some(_) => Some("No music: the folder /home/nonos/music could not be read"),
        None if tracks == 0 => {
            Some("No music yet: paste an MP3 link into Search, or put files in /home/nonos/music")
        }
        None => None,
    }
}

/// The one line the transport bar has room for, most specific first: the
/// selected track would not load, then the output failed, then the library is
/// empty.
pub fn notice(
    track: Option<&'static str>,
    output: Option<&'static str>,
    library: Option<&'static str>,
) -> Option<&'static str> {
    track.or(output).or(library)
}
