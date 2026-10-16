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

//! How long an exchange waits on the far end, judged once a tick instead of
//! inside a read loop. Two bounds, as the blocking readers had them: quiet,
//! which every byte that arrives restarts, and total, which nothing restarts,
//! so a drip of one byte every few seconds cannot hold a job open forever.

#[path = "wait_clock.rs"]
mod wait_clock;
#[path = "wait_settle.rs"]
mod wait_settle;
pub use wait_clock::Wait;
pub use wait_settle::{settle_handshake, settle_response, Settle};

/// A direct socket's quiet: a server silent this long has finished, or gone.
pub const DIRECT_QUIET_MS: i64 = 4_000;
/// A whole exchange on a direct socket.
pub const DIRECT_TOTAL_MS: i64 = 120_000;
/// Through an anonymity network seconds of quiet are normal: a reply crosses
/// several relays each way, the mixnet delaying each packet on purpose.
pub const ANON_QUIET_MS: i64 = 60_000;
/// A whole exchange through an anonymity network.
pub const ANON_TOTAL_MS: i64 = 600_000;

/// What the clock says about the far end this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Heard {
    /// Still within both bounds.
    Waiting,
    /// Nothing arrived for the quiet bound.
    Quiet,
    /// The whole exchange ran past the total bound.
    Total,
}

pub const QUIET_ANON: &str = "nothing came back for 60 seconds";
pub const QUIET_HANDSHAKE: &str = "the server went quiet during the handshake";
pub const CLOSED_HANDSHAKE: &str = "the connection closed during the handshake";
pub const TOTAL_DIRECT: &str = "gave up after 2 minutes";
pub const TOTAL_ANON: &str = "gave up after 10 minutes";
