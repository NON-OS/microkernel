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

//! A leg on the stand-in transport (crate::services::lifecycle): a driver's
//! lock and state, its requests' bytes and the replies it sends.

use core::sync::atomic::{AtomicBool, AtomicU64};

use spin::Mutex;

use super::leg::{Leg, Wire};
use crate::services::lifecycle::transport::DecodedResponse;
use crate::services::lifecycle::CapsuleState;

/// A reply as the stand-in decodes it: the request id, then the status.
pub(super) fn answer(id: u32, status: i32) -> Vec<u8> {
    [id.to_le_bytes(), status.to_le_bytes()].concat()
}

pub(super) fn decode(data: &[u8]) -> Option<DecodedResponse<'_>> {
    let id = u32::from_le_bytes(data.get(0..4)?.try_into().ok()?);
    let status = i32::from_le_bytes(data.get(4..8)?.try_into().ok()?);
    Some(DecodedResponse { op: 0, request_id: id, status, body: &[] })
}

pub(super) struct Driver {
    pub(super) lock: &'static Mutex<()>,
    pub(super) state: &'static CapsuleState,
}

pub(super) fn driver() -> Driver {
    let state = CapsuleState { alive: AtomicBool::new(true), generation: AtomicU64::new(1) };
    Driver { lock: Box::leak(Box::new(Mutex::new(()))), state: Box::leak(Box::new(state)) }
}

/// A leg for `ids`, each request its own id's bytes.
pub(super) fn leg(d: &Driver, ids: &[u32]) -> Leg {
    let wire =
        Wire { lock: d.lock, sender: "kernel", reply_inbox: "reply", state: d.state, decode };
    let requests = ids.iter().map(|&id| (id, id.to_le_bytes().to_vec())).collect();
    Leg::new("driver.test0", 1_000, wire, requests).expect("a running driver")
}
