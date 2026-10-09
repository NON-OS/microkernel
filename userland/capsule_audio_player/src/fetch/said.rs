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

//! What the bar says about a download: where it stands, and why one stopped.
//! Every refusal says what to do next.

extern crate alloc;

use alloc::format;
use alloc::string::String;

use super::Stage;

pub const ONE_AT_A_TIME: &str = "A download is already running; wait for it to finish";
pub const NO_WORKER: &str = "The download could not start; try again in a moment";
pub const NO_NETWORK: &str =
    "No network to download over: the chosen one is not running. Check Settings, then try again";
pub const NO_CONNECTION: &str =
    "The server could not be reached over the chosen network; check the address, or try again later";
pub const TLS_FAILED: &str = "A secure connection to the server could not be set up; try again later";
pub const BAD_CERT: &str =
    "The server's certificate did not verify for its name, so nothing was downloaded";
pub const NOT_HTTP: &str = "The server did not answer as a web server; check the address";
pub const DROPPED: &str =
    "The connection kept dropping, so the download stopped; try again to start it afresh";
pub const STALLED: &str = "The server stopped sending; try again later";
pub const BAD_CHUNKS: &str = "The server's answer was broken partway; try again";
pub const NO_ROOM: &str =
    "There is no room left for the file; free some space or memory, then try again";
pub const NO_FOLDER: &str = "The music folder /home/nonos/music could not be made";
pub const TOO_LARGE_TO_PLAY: &str =
    "That file is larger than 32 MiB, the most Music plays, so it was not downloaded";
pub const EMPTY: &str = "The server sent an empty file";
pub const CANCELLED: &str = "Cancelled; what came is kept, and Resume goes on from there";

/// Whether a download that stopped with `why` may go on from what came: the
/// network or the server stopped partway. A file that is not audio, too
/// large, or refused for its certificate would get the same answer again.
pub fn transient(why: &str) -> bool {
    [
        NO_WORKER,
        NO_NETWORK,
        NO_CONNECTION,
        TLS_FAILED,
        DROPPED,
        STALLED,
        BAD_CHUNKS,
        CANCELLED,
    ]
    .contains(&why)
}

/// The sentence for a file that turned out not to be audio, from what it
/// looked like instead (`nonos_download::Audio::Not`).
pub fn not_audio(kind: &str) -> &'static str {
    match kind {
        "a web page" => "That address gave a web page, not an MP3: use the link to the .mp3 file itself",
        "a JSON answer" => "That address gave a JSON answer, not an MP3: use the link to the .mp3 file itself",
        "a ZIP archive" => "That address gave a ZIP archive, not an MP3: unpack it elsewhere and use the MP3",
        "an image" => "That address gave an image, not an MP3",
        "a PDF" => "That address gave a PDF, not an MP3",
        "audio or video in a format Music does not play (only MP3 and WAV)" => {
            "That file is audio or video in a format Music does not play: only MP3 and WAV"
        }
        "a file with an ID3 tag but no MP3 audio after it" => {
            "That file has an MP3 tag but no MP3 audio after it, so it was not kept"
        }
        _ => "That address did not give an MP3, so nothing was kept",
    }
}

/// The bar's line while a download runs.
pub fn progress_line(stage: Stage, done: u64, total: Option<u64>, route: &str) -> String {
    match stage {
        Stage::Idle => String::new(),
        Stage::Connecting => format!("Downloading: connecting {route}"),
        Stage::Securing => format!("Downloading: securing the connection {route}"),
        Stage::Resuming => format!("Downloading: the connection dropped at {}, resuming", mb(done)),
        Stage::Checking => String::from("Downloading: checking it is audio"),
        Stage::Downloading => match total {
            Some(t) if t > 0 => {
                format!("Downloading {} of {} ({}%) {route}", mb(done), mb(t), done.saturating_mul(100) / t)
            }
            _ => format!("Downloading {} {route}", mb(done)),
        },
    }
}

fn mb(n: u64) -> String {
    let tenths = n.saturating_mul(10) / (1024 * 1024);
    format!("{}.{} MB", tenths / 10, tenths % 10)
}
