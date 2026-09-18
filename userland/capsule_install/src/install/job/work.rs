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

//! What a job holds while it runs: the device, the writer or the verifier,
//! the counters the screen shows, and the receipt once the write is done.

use nonos_blk_client::DeviceSink;
use nonos_disk::{Receipt, Session, Verifier};

pub enum Phase {
    Writing(Session<'static>),
    Verifying(Verifier<'static>),
}

pub struct Job {
    pub sink: DeviceSink,
    pub phase: Phase,
    pub receipt: Option<Receipt<'static>>,
    pub done: u64,
    pub total: u64,
    pub started_ms: u64,
    pub write_seconds: u64,
}

impl Job {
    pub fn writing(sink: DeviceSink, session: Session<'static>, now_ms: u64) -> Job {
        Job {
            sink,
            total: session.total_bytes(),
            phase: Phase::Writing(session),
            receipt: None,
            done: 0,
            started_ms: now_ms,
            write_seconds: 0,
        }
    }

    /// Bytes per tick. Two MiB is sixty-four driver requests, which keeps a
    /// frame under the compositor's period on the slowest path measured
    /// (TCG under a hypervisor) while moving the shipped image in a few
    /// hundred steps.
    pub const BUDGET: usize = 2 << 20;
}
