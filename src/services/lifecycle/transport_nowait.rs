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

//! Capsule IPC that never waits (the entropy ask): a request posted and
//! the caller gone on, and the replies that have landed taken on a later
//! call. `round_trip`'s checks, split where it would wait: the capsule
//! alive and its generation captured at the post, and a reply taken under
//! another generation refused as stale. Matching the request id is the
//! caller's, which decodes the reply in place, so its payload is wiped when
//! the message drops (C8).

extern crate alloc;

use alloc::format;

use super::state::CapsuleState;
use super::transport::TransportError;
use crate::ipc::nonos_channel::IpcMessage;
use crate::ipc::nonos_inbox;

/// `request` into `state`'s capsule's inbox from `sender_name`, the
/// capsule woken; the generation it went to.
pub fn post(
    request: &[u8],
    sender_name: &str,
    state: &CapsuleState,
) -> Result<u64, TransportError> {
    let gen_at_send = state.generation();
    if !state.is_alive() {
        return Err(TransportError::Dead);
    }
    let target = format!("proc.{}", state.pid());
    let msg = IpcMessage::new(sender_name, &target, request)
        .map_err(|_| TransportError::TransportFailure)?;
    match nonos_inbox::try_enqueue_strict(&target, msg) {
        Ok(()) => {
            crate::sched::wake_process(state.pid());
            Ok(gen_at_send)
        }
        Err(nonos_inbox::StrictEnqueueError::QueueFull(_)) => Err(TransportError::TransportFailure),
        Err(_) => Err(TransportError::Dead),
    }
}

/// The next reply landed in `reply_inbox`, None when none waits; refused as
/// stale when `state`'s capsule is no longer the generation `gen_at_send`.
pub fn landed(
    reply_inbox: &str,
    state: &CapsuleState,
    gen_at_send: u64,
) -> Option<Result<IpcMessage, TransportError>> {
    let reply = nonos_inbox::try_dequeue_existing(reply_inbox)?;
    if state.generation() != gen_at_send {
        return Some(Err(TransportError::Stale));
    }
    Some(Ok(reply))
}
