// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! One disk driver's part of the leave phase: its lock taken without
//! waiting (another caller mid-request holds it, and the leg waits its turn
//! inside its budget), its requests posted one at a time, each reply picked
//! out by request id. The lock stays held until the leg drops, so no other
//! kernel caller can take a reply or send behind it. A driver serving
//! several disks (the block wire's units) has one request per disk, all
//! inside the one budget; the leg answers the first failure, or 0.

use alloc::vec::Vec;

use spin::{Mutex, MutexGuard};

use crate::services::lifecycle::transport::DecodedResponse;
use crate::services::lifecycle::CapsuleState;

pub(crate) type Decode = fn(&[u8]) -> Option<DecodedResponse<'_>>;

pub(crate) struct Leg {
    /// The driver as the leave lines name it: "driver.nvme0".
    pub name: &'static str,
    pub budget_ms: u64,
    pub(super) wire: Wire,
    /// Each request with its id, the next to post first.
    pub(super) requests: Vec<(u32, Vec<u8>)>,
    pub(super) next: usize,
    pub(super) failed: i32,
    pub(super) held: Option<MutexGuard<'static, ()>>,
    /// The generation the request went to, once posted.
    pub(super) posted: Option<u64>,
}

/// A driver's wire to the kernel, as its client mirror holds it.
pub(crate) struct Wire {
    pub lock: &'static Mutex<()>,
    pub sender: &'static str,
    pub reply_inbox: &'static str,
    pub state: &'static CapsuleState,
    pub decode: Decode,
}

impl Leg {
    /// A leg for a running driver, or None when it is not running.
    pub(crate) fn new(
        name: &'static str,
        budget_ms: u64,
        wire: Wire,
        requests: Vec<(u32, Vec<u8>)>,
    ) -> Option<Self> {
        if !wire.state.is_alive() || requests.is_empty() {
            return None;
        }
        Some(Self { name, budget_ms, wire, requests, next: 0, failed: 0, held: None, posted: None })
    }
}
