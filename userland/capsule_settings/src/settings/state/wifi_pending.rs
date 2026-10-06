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

//! A scan or a join the person asked for, carried out from the app's tick
//! rather than inside the key press that asked.
//!
//! Both are one request to the Wi-Fi driver that answers only when the radio
//! work is done (a scan sweeps the channels, a join runs association and the
//! four-way handshake), and the answer is the result itself: the networks,
//! or the join's status and counters. A call is the only way to receive it,
//! so the call waits, on a worker thread (`wifi_worker.rs`) rather than the
//! window's. The key press only records the request. The next tick paints
//! "Scanning..." or "Joining <name>..." on the window, and the tick after
//! hands the call to the worker; the request is then out, and stays on the
//! panel, until a later tick finds the answer. A request no tick has picked
//! up in `STALE_MS` (the window was hidden, say) is dropped rather than run
//! long after it was asked for. A request already out is not dropped: the
//! driver's own budget (15 s for a scan, 30 s for a join) ends it.

/// What was asked for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Work {
    Scan,
    Join,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pending {
    Idle,
    /// Asked at `since_ms`; `shown` once a tick has painted that it waits.
    Asked {
        work: Work,
        since_ms: i64,
        shown: bool,
    },
    /// Handed to the worker, whose answer has not come back.
    Out(Work),
}

/// What this tick should do.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Next {
    Nothing,
    /// Repaint, so the waiting state is on screen before the call.
    Show,
    /// Run the call now.
    Run(Work),
    /// Too old to run; drop it and say so.
    Expired(Work),
}

/// A request older than this when its turn comes is dropped.
pub const STALE_MS: i64 = 10_000;

impl Pending {
    pub fn ask(work: Work, now_ms: i64) -> Self {
        Pending::Asked { work, since_ms: now_ms, shown: false }
    }

    /// What is waiting, if anything, for the panel to say.
    pub fn work(self) -> Option<Work> {
        match self {
            Pending::Idle => None,
            Pending::Asked { work, .. } | Pending::Out(work) => Some(work),
        }
    }

    /// Whether a worker has the request: the driver is busy with it, so
    /// nothing else may be asked of the driver until it answers.
    pub fn is_out(self) -> bool {
        matches!(self, Pending::Out(_))
    }

    /// Whether a request waits for a tick to show or run it, which wants
    /// the ticks to come quickly.
    pub fn is_asked(self) -> bool {
        matches!(self, Pending::Asked { .. })
    }

    /// Advance one tick. A request is shown on the first tick after it was
    /// asked and run on the next, never both in one. One that is out waits
    /// for its answer (`Nothing`); the caller looks for that itself.
    pub fn tick(&mut self, now_ms: i64) -> Next {
        let Pending::Asked { work, since_ms, shown } = *self else {
            return Next::Nothing;
        };
        if now_ms.saturating_sub(since_ms) >= STALE_MS {
            *self = Pending::Idle;
            return Next::Expired(work);
        }
        if !shown {
            *self = Pending::Asked { work, since_ms, shown: true };
            return Next::Show;
        }
        *self = Pending::Idle;
        Next::Run(work)
    }
}
