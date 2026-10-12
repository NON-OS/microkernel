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

//! When autojoin acts, kept free of the kernel and the driver so the host
//! proofs run it unchanged. The glue in `tick` makes the calls; this decides
//! which one is due and what each outcome leads to.
//!
//! A join takes the driver seconds (the hunt, then authentication and the
//! four-way handshake), and net_core's one loop serves every TCP, UDP, DHCP
//! and DNS client, so autojoin never waits for one: it hands the driver the
//! request with a short wait, and if no answer came in that time the join runs
//! on in the driver while autojoin watches the link once a second until it is
//! associated or `JOIN_WATCH_MS` has passed. Each saved network is tried at
//! most once per boot, so a wrong passphrase is not replayed at the access
//! point. A link that was up and goes down is joined again (`rejoin`).

#[path = "machine/after.rs"]
mod after;
#[path = "machine/again.rs"]
mod again;
#[path = "machine/consts.rs"]
mod consts;
#[path = "machine/rejoin.rs"]
mod rejoin;
#[path = "machine/state.rs"]
mod state;
#[path = "machine/step.rs"]
mod step;
#[path = "machine/watch.rs"]
mod watch;

pub use consts::*;
pub use state::Machine;
use step::Phase;
pub use step::{Due, Step};
