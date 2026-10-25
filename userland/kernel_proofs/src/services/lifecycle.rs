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

//! The kernel's capsule lifecycle as a leave leg reaches it
//! (src/hardware/block_device/leave/leg_poll.rs): a capsule's liveness and
//! generation, a post into its inbox and the replies that land, with the
//! test holding each. Shaped as src/services/lifecycle/transport_nowait.rs:
//! a post to a dead capsule is Dead, one with no room is TransportFailure,
//! and a reply landing under another generation is Stale.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub mod transport {
    pub struct DecodedResponse<'a> {
        pub op: u16,
        pub request_id: u32,
        pub status: i32,
        pub body: &'a [u8],
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TransportError {
        Dead,
        Stale,
        TransportFailure,
        ProtocolMismatch,
    }
}

pub mod transport_nowait;

pub struct CapsuleState {
    pub alive: AtomicBool,
    pub generation: AtomicU64,
}

impl CapsuleState {
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
    }

    pub fn generation(&self) -> u64 {
        self.generation.load(Ordering::SeqCst)
    }

    /// A restart: a new generation.
    pub fn restart(&self) {
        self.generation.fetch_add(1, Ordering::SeqCst);
    }
}

pub struct IpcMessage {
    pub data: alloc::vec::Vec<u8>,
}
