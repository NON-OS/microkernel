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

use crate::transport::Transport;
use nonos_bt_wire::addr::AddrType;

/// Polls one command may wait. With the capsule's millisecond pause this
/// is about two seconds: a controller answers a command in milliseconds,
/// and Reset, the slowest, well inside that.
pub const POLL_BUDGET: u32 = 2_000;

pub struct Host<T> {
    pub(super) t: T,
    /// Num_HCI_Command_Packets as last told; the host may send one command
    /// before it is told anything (Vol 4, Part E, 4.4).
    pub(super) credits: u8,
    pub(super) budget: u32,
    pub(super) queue: VecDeque<Vec<u8>>,
    pub(super) dropped: u32,
    /// The controller's ACL packet length and count, and how many of those
    /// buffers are free now (Vol 4, Part E, 4.1.1). Zero until bring-up.
    pub(super) acl_len: u16,
    pub(super) acl_total: u16,
    pub(super) acl_free: u16,
    /// The address type scans and connections are made as (7.8.4).
    pub(super) own: AddrType,
    /// The Scanning_Filter_Policy scans use, chosen at bring-up (7.8.10).
    pub(super) scan_filter: u8,
}

impl<T: Transport> Host<T> {
    pub fn new(t: T) -> Self {
        Self::with_budget(t, POLL_BUDGET)
    }

    pub fn with_budget(t: T, budget: u32) -> Self {
        let queue = VecDeque::new();
        let (acl_len, acl_total, acl_free) = (0, 0, 0);
        let own = AddrType::Public;
        let scan_filter = super::scan::ACCEPT_ALL;
        let dropped = 0;
        Self {
            t,
            credits: 1,
            budget,
            queue,
            dropped,
            acl_len,
            acl_total,
            acl_free,
            own,
            scan_filter,
        }
    }
}
