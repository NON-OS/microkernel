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

//! A host on a transport, before bring-up.

use alloc::collections::VecDeque;
use alloc::vec::Vec;

use super::state::{Host, POLL_BUDGET};
use crate::transport::Transport;
use nonos_bt_wire::addr::AddrType;

impl<T: Transport> Host<T> {
    pub fn new(t: T) -> Self {
        Self::with_budget(t, POLL_BUDGET)
    }

    pub fn with_budget(t: T, budget: u32) -> Self {
        let (queue, dropped, credits) = (VecDeque::new(), 0, 1);
        let (acl_len, acl_total, acl_free) = (0, 0, 0);
        let (own, scan_filter) = (AddrType::Public, super::scan::ACCEPT_ALL);
        let (data_length, held, resetting) = (None, Vec::new(), false);
        Self {
            t,
            credits,
            budget,
            queue,
            dropped,
            acl_len,
            acl_total,
            acl_free,
            own,
            scan_filter,
            data_length,
            held,
            resetting,
        }
    }
}
