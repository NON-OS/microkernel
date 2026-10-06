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

//! One download, start to end, on the worker thread: connect over the chosen
//! route, TLS with the chain checked for the host, the request, the answer
//! judged (`nonos_download::decide`), the body written to /tmp as it comes,
//! resumed from where it stopped when the connection drops, checked as audio,
//! and moved into the music folder.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::{self, VfsStream, VfsWriter};
use nonos_download::{
    audio_start, decide, parse_head, request, Audio, Chunked, Decision, DlUrl, MAX_BYTES,
};
use nonos_libc::{mk_getpid, mk_uptime_ms};
use nonos_route_link::{Route, RouteStream};
use nonos_tls::stream::{connect, Stream};
use nonos_tls::{rtc_now, SessionError};

use super::io::RouteIo;
use super::name::{file_name, free_path, MUSIC_DIR};
use super::said::*;
use super::{set_done, set_stage, set_total, Stage};
use crate::track_limit::MAX_FILE;

/// Connections a download may take: the first, then resumes after drops.
const TRIES: u32 = 6;
/// No byte for this long and the connection is given up for a resume.
const IDLE_MS: i64 = 60_000;
/// The longest response head read.
const HEAD_MAX: usize = 32 * 1024;
/// Bytes kept from the start of the file for the audio check.
const LEAD: usize = 64 * 1024;

/// How one connection ended.
enum Step {
    Redirect(DlUrl),
    Whole,
    Dropped,
    Refuse(&'static str),
}

/// Download `url` into `part`, going on from what `part` already holds, and
/// move it into the music folder once it is whole and audio. On a stop that
/// may be gone on from (`said::transient`) the part is kept; otherwise it is
/// deleted.
pub fn download(url: DlUrl, part: &str) -> super::Outcome {
    let pid = mk_getpid();
    attempt(pid, url, part).map_err(|why| {
        let keep = transient(why);
        if !keep {
            let _ = vfs::unlink(pid, part.as_bytes());
        }
        (why, keep)
    })
}

fn attempt(pid: u32, mut url: DlUrl, part: &str) -> Result<String, &'static str> {
    let route = match Route::chosen() {
        Route::Down(_) => return Err(NO_NETWORK),
        r => r,
    };
    super::set_route(route.name());
    for dir in ["/tmp", "/home", "/home/nonos", MUSIC_DIR] {
        let _ = vfs::mkdir(pid, dir.as_bytes());
    }
    let mut file = VfsWriter::reopen(pid, part.as_bytes()).map_err(|_| NO_ROOM)?;
    // What came before, for the audio check, which reads the file's start.
    let mut lead: Vec<u8> = Vec::new();
    if file.written() > 0 {
        let mut s = VfsStream::open(pid, part.as_bytes()).map_err(|_| NO_ROOM)?;
        lead = s.read_window(0, LEAD as u32).map_err(|_| NO_ROOM)?;
        set_done(file.written());
    }
    let mut redirects = 0u8;
    let mut tries = 0u32;
    let mut total = None;
    let outcome = loop {
        if super::cancelled() {
            break Err(CANCELLED);
        }
        match fetch(route, &url, &mut file, &mut lead, &mut total, redirects) {
            Step::Redirect(next) => {
                redirects += 1;
                url = next;
            }
            Step::Whole => break Ok(()),
            Step::Dropped => {
                tries += 1;
                if tries >= TRIES {
                    break Err(DROPPED);
                }
                set_stage(Stage::Resuming);
            }
            Step::Refuse(why) => break Err(why),
        }
    };
    let ext = outcome.and_then(|()| check(pid, part, &lead, file.written()));
    drop(file);
    let ext = ext?;
    let name = file_name(&url.target, ext);
    let dest = free_path(&name, |p| vfs::stat(pid, p.as_bytes()).is_ok());
    if vfs::rename(pid, part.as_bytes(), dest.as_bytes()).is_err() {
        return Err(NO_FOLDER);
    }
    Ok(dest)
}

/// One connection: from the byte the file has reached.
fn fetch(
    route: Route,
    url: &DlUrl,
    file: &mut VfsWriter,
    lead: &mut Vec<u8>,
    total: &mut Option<u64>,
    redirects: u8,
) -> Step {
    set_stage(Stage::Connecting);
    let Ok(stream) = RouteStream::connect(route, &url.host, url.port) else {
        return if file.written() > 0 { Step::Dropped } else { Step::Refuse(NO_CONNECTION) };
    };
    let mut io = RouteIo(stream);
    set_stage(Stage::Securing);
    let mut tls = match connect(&mut io, url.host.as_bytes(), rtc_now()) {
        Ok(tls) => tls,
        Err(SessionError::Certificate) => return Step::Refuse(BAD_CERT),
        Err(_) if file.written() > 0 => return Step::Dropped,
        Err(_) => return Step::Refuse(TLS_FAILED),
    };
    let from = file.written();
    if tls.write_all(&mut io, request(url, from).as_bytes()).is_err() {
        return Step::Dropped;
    }
    let (raw, mut first) = match read_head(&mut tls, &mut io) {
        Ok(h) => h,
        Err(step) => return step,
    };
    let Some(head) = parse_head(&raw) else { return Step::Refuse(NOT_HTTP) };
    let (at, length, chunked) = match decide(&head, url, from, redirects) {
        Decision::Redirect(next) => return Step::Redirect(next),
        Decision::Refuse(why) => return Step::Refuse(why),
        Decision::Body { at, total, chunked } => (at, total, chunked),
    };
    if at == 0 && from > 0 {
        if file.restart().is_err() {
            return Step::Refuse(NO_ROOM);
        }
        lead.clear();
    }
    if length.is_some_and(|n| n > u64::from(MAX_FILE)) {
        return Step::Refuse(TOO_LARGE_TO_PLAY);
    }
    if length.is_some() {
        *total = length;
    }
    set_total(*total);
    if total.is_some_and(|t| file.written() >= t) {
        return Step::Whole;
    }
    set_stage(Stage::Downloading);
    let mut chunks = chunked.then(Chunked::new);
    let mut last = mk_uptime_ms();
    loop {
        if super::cancelled() {
            return Step::Refuse(CANCELLED);
        }
        let came = if first.is_empty() {
            match tls.read(&mut io) {
                Ok(b) => b,
                Err(_) => return ended(file.written(), *total, chunks.as_ref()),
            }
        } else {
            core::mem::take(&mut first)
        };
        if came.is_empty() {
            if tls.is_done() {
                return ended(file.written(), *total, chunks.as_ref());
            }
            if mk_uptime_ms() - last > IDLE_MS {
                return if file.written() > 0 { Step::Dropped } else { Step::Refuse(STALLED) };
            }
            continue;
        }
        last = mk_uptime_ms();
        let body = match chunks.as_mut() {
            Some(c) => {
                let mut out = Vec::new();
                if c.feed(&came, &mut out).is_err() {
                    return Step::Refuse(BAD_CHUNKS);
                }
                out
            }
            None => came,
        };
        if let Some(step) = keep(file, lead, &body) {
            return step;
        }
        set_done(file.written());
        if chunks.as_ref().is_some_and(Chunked::done) || total.is_some_and(|t| file.written() >= t) {
            return Step::Whole;
        }
    }
}

/// Write `body`, held to the limits, and check the file's first bytes as
/// soon as there are enough to tell: a web page is refused at its first
/// kilobytes, not after the whole of it has come.
fn keep(file: &mut VfsWriter, lead: &mut Vec<u8>, body: &[u8]) -> Option<Step> {
    let after = file.written() + body.len() as u64;
    if after > MAX_BYTES {
        return Some(Step::Refuse(nonos_download::TOO_LARGE));
    }
    if after > u64::from(MAX_FILE) {
        return Some(Step::Refuse(TOO_LARGE_TO_PLAY));
    }
    if file.append(body).is_err() {
        return Some(Step::Refuse(NO_ROOM));
    }
    if lead.len() < LEAD {
        let room = LEAD - lead.len();
        lead.extend_from_slice(&body[..body.len().min(room)]);
        if let Audio::Not(kind) = audio_start(lead, 0) {
            return Some(Step::Refuse(not_audio(kind)));
        }
    }
    None
}

/// The connection ended: whole when the length was met, or when there was
/// no length and the body ran to the close; dropped otherwise.
fn ended(written: u64, total: Option<u64>, chunks: Option<&Chunked>) -> Step {
    match (total, chunks) {
        (_, Some(c)) if c.done() => Step::Whole,
        (_, Some(_)) => Step::Dropped,
        (Some(t), None) if written >= t => Step::Whole,
        (Some(_), None) => Step::Dropped,
        (None, None) if written > 0 => Step::Whole,
        (None, None) => Step::Dropped,
    }
}

/// The response head, and any body bytes that came with it.
fn read_head(tls: &mut Stream, io: &mut RouteIo) -> Result<(Vec<u8>, Vec<u8>), Step> {
    let mut got = Vec::new();
    let since = mk_uptime_ms();
    loop {
        if let Some(end) = got.windows(4).position(|w| w == b"\r\n\r\n") {
            let rest = got.split_off(end + 4);
            return Ok((got, rest));
        }
        if got.len() > HEAD_MAX {
            return Err(Step::Refuse(NOT_HTTP));
        }
        if super::cancelled() {
            return Err(Step::Refuse(CANCELLED));
        }
        let more = tls.read(io).map_err(|_| Step::Dropped)?;
        if more.is_empty() {
            if tls.is_done() || mk_uptime_ms() - since > IDLE_MS {
                return Err(Step::Dropped);
            }
            continue;
        }
        got.extend_from_slice(&more);
    }
}

/// The whole file read as audio: its extension, or the refusal. A tag too
/// large for the lead is followed into the file to its first frame.
fn check(pid: u32, part: &str, lead: &[u8], written: u64) -> Result<&'static str, &'static str> {
    set_stage(Stage::Checking);
    if written == 0 {
        return Err(EMPTY);
    }
    let mut verdict = audio_start(lead, 0);
    if let Audio::ReadFrom(at) = verdict {
        if at as u64 >= written {
            return Err(not_audio("a file with an ID3 tag but no MP3 audio after it"));
        }
        let mut s = VfsStream::open(pid, part.as_bytes()).map_err(|_| NO_ROOM)?;
        let want = (written - at as u64).min(8192) as u32;
        let tail = s.read_window(at as u64, want).map_err(|_| NO_ROOM)?;
        verdict = audio_start(&tail, at);
    }
    match verdict {
        Audio::Mp3 { .. } => Ok("mp3"),
        Audio::Wav => Ok("wav"),
        Audio::Not(kind) => Err(not_audio(kind)),
        Audio::Short | Audio::ReadFrom(_) => Err(not_audio("")),
    }
}
