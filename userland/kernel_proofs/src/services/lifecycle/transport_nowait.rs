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

//! `post` and `landed` on this thread's queues: what was posted, the posts
//! to refuse and with what, and the replies waiting.

use std::cell::RefCell;
use std::collections::VecDeque;

use super::transport::TransportError;
use super::{CapsuleState, IpcMessage};

thread_local! {
    static POSTED: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
    static REFUSE: RefCell<VecDeque<TransportError>> = const { RefCell::new(VecDeque::new()) };
    static REPLIES: RefCell<VecDeque<Vec<u8>>> = const { RefCell::new(VecDeque::new()) };
}

pub fn post(request: &[u8], _sender: &str, state: &CapsuleState) -> Result<u64, TransportError> {
    if let Some(e) = REFUSE.with(|r| r.borrow_mut().pop_front()) {
        return Err(e);
    }
    if !state.is_alive() {
        return Err(TransportError::Dead);
    }
    POSTED.with(|p| p.borrow_mut().push(request.to_vec()));
    Ok(state.generation())
}

pub fn landed(
    _reply_inbox: &str,
    state: &CapsuleState,
    gen_at_send: u64,
) -> Option<Result<IpcMessage, TransportError>> {
    let data = REPLIES.with(|r| r.borrow_mut().pop_front())?;
    if state.generation() != gen_at_send {
        return Some(Err(TransportError::Stale));
    }
    Some(Ok(IpcMessage { data }))
}

/// Refuse the next post with `e`.
pub fn refuse_next(e: TransportError) {
    REFUSE.with(|r| r.borrow_mut().push_back(e));
}

/// A reply waiting in the inbox.
pub fn reply(data: Vec<u8>) {
    REPLIES.with(|r| r.borrow_mut().push_back(data));
}

/// Every request posted on this thread, in order.
pub fn posted() -> Vec<Vec<u8>> {
    POSTED.with(|p| p.borrow().clone())
}
