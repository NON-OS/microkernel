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

//! Which pending entry a reply answers. A direct mk_ipc_reply: the request
//! the server dequeued last when it is the caller's, else the caller's
//! oldest dequeued one. A redirect: the request dequeued last. An
//! MkIpcReplyTo: the dequeued request it names. Never one still in the
//! inbox, and never one already answered. With the request ids and the
//! meta MkIpcRecvReq hands the server, held to abi/wire.toml.

#[path = "../../../../src/syscall/microkernel/ipc/recv_meta.rs"]
mod recv_meta;
#[path = "../../../../src/syscall/microkernel/ipc/pending_reply/reply_rule.rs"]
mod reply_rule;
#[path = "../../../../src/syscall/microkernel/ipc/pending_reply/request_ids.rs"]
mod request_ids;
#[path = "../../../../src/syscall/microkernel/ipc/pending_reply/served_rule.rs"]
mod served_rule;

/// The entry ipc_death mounts, so the file is one module.
use crate::proofs::proofs_8::ipc_death::entry;

mod answered_tests;
mod meta_tests;
mod model;
mod redirect_tests;
mod reply_to_tests;
mod sequence_tests;
mod tests;
