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

//! `install` waits on the installer only briefly and follows a load still
//! running from the job's ticks: the new child starts the drain, an
//! installer that answers a probe with no child fails it, the deadline ends
//! it, and probes back off so the terminal holds fewer than the kernel's
//! eight places in the installer's reply queue.

mod glue;
mod outcome;
mod pacing;

use crate::install_follow::Follow;

fn fresh() -> Follow {
    Follow::new(vec![7, 9], 10_000)
}
