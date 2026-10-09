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

//! Whether a delivery came from the input router.
//!
//! Any process that may use IPC may send a frame to any pid's inbox, and an
//! input frame is recognised by its magic alone. Without this check a capsule
//! could type into whichever app it liked. The kernel records the true sender,
//! so the router's pid is what separates routed input from forged input.

use core::sync::atomic::{AtomicU32, Ordering};

use super::lookup_service::lookup_service;

static ROUTER_PID: AtomicU32 = AtomicU32::new(0);

/// True only when `sender` is the pid that owns the `input_router` service.
/// A router that restarted is looked up again before a frame is refused.
pub fn from_router(sender: u32) -> bool {
    let known = ROUTER_PID.load(Ordering::Acquire);
    if known != 0 && known == sender {
        return true;
    }
    let Some(router) = lookup_service(b"input_router") else {
        return false;
    };
    ROUTER_PID.store(router.pid, Ordering::Release);
    router.pid == sender
}
