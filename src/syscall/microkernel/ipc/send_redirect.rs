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

//! Where a send goes: the endpoint it names, and whether it is an answer
//! on the sender's own reply port.

extern crate alloc;

use crate::services::registry::{lookup_port, lookup_service};
use crate::syscall::microkernel::narrow::u32_arg;

pub(super) enum Redirect {
    ToCaller { caller_inbox: alloc::string::String, caller_pid: u32, token: u64 },
    ToReplyInbox,
    AsAddressed,
}

// Classify a send. A capsule sending to its own fixed reply endpoint is
// answering a request, and which request decides where the bytes go: a pending
// mk_ipc_call caller takes them (ToCaller), otherwise the kernel is the one
// waiting on this inbox for a round trip it drove, so they stay here for the
// drain (ToReplyInbox). Every other send is addressed as written (AsAddressed).
pub(super) fn redirect_reply(sender_pid: u32, target: &str) -> Redirect {
    let own_reply = crate::process::get_process(sender_pid).and_then(|p| p.reply_inbox());
    if own_reply.as_ref().map(|n| n.as_str()) == Some(target) {
        if let Some((caller_pid, caller_inbox, token)) = super::pending_reply::pop(sender_pid) {
            return Redirect::ToCaller { caller_inbox, caller_pid, token };
        }
        return Redirect::ToReplyInbox;
    }
    Redirect::AsAddressed
}

pub(super) fn resolve_send_target(endpoint: u64) -> alloc::string::String {
    let numeric = alloc::format!("endpoint.{}", endpoint);
    if lookup_service(&numeric).is_some() {
        return numeric;
    }
    u32_arg(endpoint).and_then(lookup_port).map(|ep| ep.name).unwrap_or(numeric)
}
