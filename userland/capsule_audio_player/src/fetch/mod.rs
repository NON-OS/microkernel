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

//! An MP3 downloaded from an address the person types into Search, over the
//! network the system routes everything through (Anyone unless they chose
//! otherwise), and played.
//!
//! The download runs on one worker thread from its first call to its last
//! (libc's `WorkerSeat`, as Settings asks the Wi-Fi driver): net.anon holds a
//! stream for the thread that opened it, and vfs a file descriptor for the
//! thread that opened it, so every call of one download is made from one
//! thread. The window keeps painting; a tick reads the progress here and
//! picks the outcome up from the `Handoff`.
//!
//! The file is written to /tmp as it comes and moved into /home/nonos/music
//! only once it is whole and has been read as audio, so a refused download
//! never appears in the music folder.

extern crate alloc;

use alloc::string::String;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering::Relaxed};

use nonos_download::DlUrl;
use nonos_libc::{Handoff, Look, WorkerSeat};
use spin::Mutex;

mod io;
mod job;
pub mod list;
pub mod name;
pub mod row_text;
mod said;

pub use said::transient;

/// How a download ended: the file's path in the music folder, or why there
/// is none and whether what came is kept to go on from.
pub type Outcome = Result<String, (&'static str, bool)>;


/// Where the download stands, for the bar.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Stage {
    Idle = 0,
    Connecting = 1,
    Securing = 2,
    Downloading = 3,
    Resuming = 4,
    Checking = 5,
}

static STAGE: AtomicU8 = AtomicU8::new(0);
static DONE: AtomicU64 = AtomicU64::new(0);
/// The file's length, 0 when the server did not say.
static TOTAL: AtomicU64 = AtomicU64::new(0);
/// The address the worker is to fetch and the part file it writes, handed
/// over when it starts.
static ASKED: Mutex<Option<(DlUrl, String)>> = Mutex::new(None);
/// The person asked the running download to stop.
static CANCEL: AtomicBool = AtomicBool::new(false);
/// The network the download goes over, in the words the bar uses.
static ROUTE: Mutex<&'static str> = Mutex::new("");
static ANSWER: Handoff<Outcome> = Handoff::new();
static SEAT: WorkerSeat = WorkerSeat::new();

pub(crate) fn set_stage(s: Stage) {
    STAGE.store(s as u8, Relaxed);
}

pub(crate) fn set_done(n: u64) {
    DONE.store(n, Relaxed);
}

pub(crate) fn set_total(n: Option<u64>) {
    TOTAL.store(n.unwrap_or(0), Relaxed);
}

pub(crate) fn set_route(name: &'static str) {
    *ROUTE.lock() = name;
}

/// The bar's line while a download runs, empty when none is.
pub fn line() -> String {
    let (stage, done, total) = progress();
    said::progress_line(stage, done, total, *ROUTE.lock())
}

/// The stage, bytes so far, and the length when known.
pub fn progress() -> (Stage, u64, Option<u64>) {
    let stage = match STAGE.load(Relaxed) {
        1 => Stage::Connecting,
        2 => Stage::Securing,
        3 => Stage::Downloading,
        4 => Stage::Resuming,
        5 => Stage::Checking,
        _ => Stage::Idle,
    };
    let total = TOTAL.load(Relaxed);
    (stage, DONE.load(Relaxed), (total > 0).then_some(total))
}

/// Whether `text` is an address to download rather than words to search.
pub fn is_address(text: &str) -> bool {
    let t = text.trim();
    t.starts_with("https://") || t.starts_with("http://")
}

/// Start downloading `url` into `part`, going on from what `part` holds. The
/// refusal, said plainly, when it cannot start.
pub fn start(url: DlUrl, part: String) -> Result<(), &'static str> {
    if !ANSWER.claim() {
        return Err(said::ONE_AT_A_TIME);
    }
    CANCEL.store(false, Relaxed);
    *ASKED.lock() = Some((url, part));
    set_stage(Stage::Connecting);
    set_done(0);
    set_total(None);
    set_route("");
    match SEAT.spawn(work, 0) {
        Ok(_) => Ok(()),
        Err(_) => {
            ANSWER.release();
            set_stage(Stage::Idle);
            Err(said::NO_WORKER)
        }
    }
}

/// Ask the running download to stop. It stops at its next read, within a
/// couple of seconds, and keeps what came.
pub fn cancel() {
    CANCEL.store(true, Relaxed);
}

/// Whether the person asked the running download to stop.
pub(crate) fn cancelled() -> bool {
    CANCEL.load(Relaxed)
}

/// The outcome once the download has ended.
pub fn outcome() -> Option<Outcome> {
    match ANSWER.poll() {
        Look::Ready(answer) => {
            set_stage(Stage::Idle);
            Some(answer)
        }
        Look::Waiting | Look::Idle => None,
    }
}

fn work(_: usize) {
    let asked = ASKED.lock().take();
    let answer = match asked {
        Some((url, part)) => job::download(url, &part),
        None => Err((said::NO_WORKER, true)),
    };
    ANSWER.put(answer);
}
