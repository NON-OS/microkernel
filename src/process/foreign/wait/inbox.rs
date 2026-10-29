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

//! How many messages each supervisor's own inbox had taken when its wait was
//! last told of one (wake_rule.rs). The inbox is the one MkIpcRecvFrom reads
//! as endpoint 0, `proc.<pid>`, and a delivery there wakes the process.

use alloc::collections::BTreeMap;
use alloc::format;
use spin::Mutex;

use super::wake_rule::{look as rule, Look};
use crate::ipc::nonos_inbox;

static HEARD: Mutex<BTreeMap<u32, u64>> = Mutex::new(BTreeMap::new());

/// One look for `caller`, `trapped` when a frame was claimed for it. Told of
/// the inbox, the count is kept, so the next look sleeps until another comes.
pub(super) fn look(caller: u32, trapped: bool) -> Look {
    let own = format!("proc.{caller}");
    let enqueued = nonos_inbox::get_inbox_stats(&own).map_or(0, |s| s.enqueued);
    let mut heard = HEARD.lock();
    let found = rule(trapped, enqueued, heard.get(&caller).copied().unwrap_or(0));
    if found == Look::Inbox {
        heard.insert(caller, enqueued);
    }
    found
}

/// `pid` is gone (clear.rs): a reused pid starts from its new inbox.
pub(crate) fn forget(pid: u32) {
    HEARD.lock().remove(&pid);
}
