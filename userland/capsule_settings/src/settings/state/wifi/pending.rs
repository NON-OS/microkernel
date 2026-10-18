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
//! so the call waits, on a worker thread (`state/wifi/worker.rs`) rather than
//! the window's. The key press only records the request (`pending_tick`).

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
}
