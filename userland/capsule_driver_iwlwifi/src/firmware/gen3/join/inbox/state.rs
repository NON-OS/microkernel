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
//! What the inbox holds, and the access point's transmit queues.

use super::super::super::rx_data::RxMpdu;
use super::super::super::station::session::Session;
use super::super::super::txq::TxQueue;
use alloc::collections::VecDeque;
/// Received frames held for the join or the link.
pub const INBOX_FRAMES: usize = 64;

/// The access point's two transmit queues.
pub struct Queues {
    pub mgmt: TxQueue,
    pub data: TxQueue,
}

#[derive(Default)]
pub struct Inbox {
    pub frames: VecDeque<RxMpdu>,
    /// The last session protection event not yet read.
    pub session: Option<Session>,
    /// Frames dropped with the queue full.
    pub dropped: u32,
    /// Transmit responses that named no queue of the access point's, or more
    /// slots than were in flight.
    pub stray: u32,
    /// Frames the firmware answered as not sent.
    pub tx_failed: u32,
}

impl Inbox {
    pub fn new() -> Self {
        Self::default()
    }
}
