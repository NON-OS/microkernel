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

//! The Wi-Fi scan and join, made on a worker thread.
//!
//! Each is one call to the driver that answers only when the radio work is
//! done (up to 15 s for a scan, 30 s for a join), and the answer is the reply
//! itself, so the call has to wait. A worker waits for it while the window
//! keeps painting and taking keys; a tick picks the answer up (`wifi_step`).
//! One worker at a time: the driver is busy with it, so nothing else is asked
//! of the driver until it answers (`Pending::is_out`).
//!
//! The kernel admits a caller to the Wi-Fi drivers when its process owns the
//! Settings endpoint, and judges a thread by its process (4e4c172a), so the
//! worker is admitted where the window thread is.
//!
//! The passphrase of a join travels in the request, in a buffer wiped when
//! the request is dropped, whichever thread drops it and however the join
//! went. Remembering the network (sealing it with the TPM key) happens on the
//! worker too, while it still holds the passphrase; the answer carries only
//! the outcome, never the passphrase. The answer lands in a static, so a
//! worker that finishes after Settings closed writes into memory still there.

use alloc::boxed::Box;

use nonos_libc::{Handoff, Look, WorkerSeat};
use nonos_wifi_client::{remember, wipe, Driver};

use super::cache::STRING_CAP;
use super::state::WIFI_NET_MAX;
use crate::wifi::{ConnectResult, ScanNetwork, ScanOutcome, ScanStats};

/// A passphrase on its way to the driver, wiped when dropped.
pub(super) struct Secret {
    bytes: [u8; STRING_CAP],
    len: usize,
}

impl Secret {
    pub(super) fn new(from: &[u8]) -> Self {
        let mut s = Self { bytes: [0u8; STRING_CAP], len: from.len().min(STRING_CAP) };
        s.bytes[..s.len].copy_from_slice(&from[..s.len]);
        s
    }

    fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl Drop for Secret {
    fn drop(&mut self) {
        wipe(&mut self.bytes);
        self.len = 0;
    }
}

/// What the worker asks the driver.
pub(super) enum Ask {
    Scan(Driver),
    /// Join `net`; keep it among the saved networks when `remember`.
    Join {
        driver: Driver,
        net: ScanNetwork,
        pass: Secret,
        remember: bool,
    },
}

/// The driver's answer.
pub(super) enum Answer {
    Scanned {
        nets: [ScanNetwork; WIFI_NET_MAX],
        count: usize,
        outcome: ScanOutcome,
        stats: ScanStats,
    },
    /// The join's outcome and, when remembering was asked for and the join
    /// went through, what remembering said.
    Joined { result: ConnectResult, kept: Option<&'static str> },
}

/// What became of a request handed to `send`.
pub(super) enum Sent {
    /// A worker has it; a tick will find the answer.
    Out,
    /// The last request is still with the driver; this one was dropped.
    Busy,
    /// No worker could be started (the kernel refused the thread, or its stack
    /// could not be mapped): the request comes back, for
    /// the window thread to run itself.
    Refused(Box<Ask>),
}

static ANSWER: Handoff<Answer> = Handoff::new();
static SEAT: WorkerSeat = WorkerSeat::new();

/// The seat's refusal while its last worker is still on it.
const EBUSY: i64 = -16;

/// Hand `ask` to a worker.
pub(super) fn send(ask: Box<Ask>) -> Sent {
    if !ANSWER.claim() {
        return Sent::Busy;
    }
    let arg = Box::into_raw(ask);
    match SEAT.spawn(work, arg as usize) {
        Ok(_) => Sent::Out,
        Err(errno) => {
            // SAFETY: the spawn failed, so no worker took the box; it is
            // still this thread's.
            let ask = unsafe { Box::from_raw(arg) };
            ANSWER.release();
            if errno == EBUSY {
                // The last worker has answered but is not gone yet.
                return Sent::Busy;
            }
            Sent::Refused(ask)
        }
    }
}

/// The answer, once a worker has put it in.
pub(super) fn poll() -> Look<Answer> {
    ANSWER.poll()
}

fn work(arg: usize) {
    // SAFETY: `arg` is the box `send` leaked for this worker, and the spawn
    // succeeded, so the worker owns it from here; dropping it wipes the
    // passphrase.
    let ask = unsafe { Box::from_raw(arg as *mut Ask) };
    let answer = perform(&ask);
    drop(ask);
    ANSWER.put(answer);
}

/// Ask the driver, on whichever thread calls this.
pub(super) fn perform(ask: &Ask) -> Answer {
    match ask {
        Ask::Scan(driver) => {
            let mut nets = [ScanNetwork::EMPTY; WIFI_NET_MAX];
            let (count, outcome, stats) = driver.scan(&mut nets);
            Answer::Scanned { nets, count, outcome, stats }
        }
        Ask::Join { driver, net, pass, remember: keep } => {
            let result = driver.connect(net.ssid(), pass.as_slice());
            let kept =
                (result.code == 0 && *keep).then(|| match remember(net.ssid(), pass.as_slice()) {
                    Ok(()) => "Remembered, sealed with this machine's TPM key",
                    Err(e) => e.text(),
                });
            Answer::Joined { result, kept }
        }
    }
}
