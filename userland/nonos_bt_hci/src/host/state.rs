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

//! The host over one transport: how many commands and ACL packets the
//! controller takes now, and the events that came while a command was
//! being waited for.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use nonos_bt_wire::addr::AddrType;

/// Polls one command may wait. With the capsule's millisecond pause this
/// is about two seconds: a controller answers a command in milliseconds,
/// and Reset, the slowest, well inside that.
pub const POLL_BUDGET: u32 = 2_000;

/// The host issues one command at a time and waits for its answer, so it
/// never has more than one in flight. Num_HCI_Command_Packets is clamped
/// to this, so a controller granting more cannot inflate the host's count
/// (Vol 4, Part E, 4.4).
pub const COMMAND_CREDITS_MAX: u8 = 1;

pub struct Host<T> {
    pub(super) t: T,
    /// Num_HCI_Command_Packets as last told, clamped to COMMAND_CREDITS_MAX;
    /// the host may send one command before it is told anything (4.4).
    pub(super) credits: u8,
    pub(super) budget: u32,
    pub(super) queue: VecDeque<Vec<u8>>,
    pub(super) dropped: u32,
    /// The controller's ACL packet length and count, and how many of those
    /// buffers are free now (Vol 4, Part E, 4.1.1). Zero until bring-up.
    pub(super) acl_len: u16,
    pub(super) acl_total: u16,
    pub(super) acl_free: u16,
    /// The buffers each link holds now, by handle; only links holding any.
    pub(super) held: Vec<(u16, u16)>,
    /// Set while Reset is waited for, when a Hardware Error or another
    /// Reset's completion is the past being cleared, not news.
    pub(super) resetting: bool,
    /// The address type scans and connections are made as (7.8.4).
    pub(super) own: AddrType,
    /// The Scanning_Filter_Policy scans use, chosen at bring-up (7.8.10).
    pub(super) scan_filter: u8,
    /// The PDU octets and time to ask each connection for, from bring-up.
    pub(super) data_length: Option<(u16, u16)>,
}

impl<T> Host<T> {
    /// Command credits held now, never above COMMAND_CREDITS_MAX.
    pub fn credits(&self) -> u8 {
        self.credits
    }
}
