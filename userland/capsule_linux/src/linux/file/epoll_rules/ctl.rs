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

//! Which event masks EPOLL_CTL refuses. Pure, so the host proofs hold it.

use super::look::{
    EPOLLERR, EPOLLEXCLUSIVE, EPOLLHUP, EPOLLIN, EPOLLOUT, EPOLLWAKEUP, EPOLL_CTL_ADD,
    EPOLL_CTL_MOD,
};
use crate::linux::abi::errno;
use crate::linux::guest::EPOLLET;

/// The bits Linux allows beside EPOLLEXCLUSIVE.
const EXCLUSIVE_OK: u32 =
    EPOLLIN | EPOLLOUT | EPOLLERR | EPOLLHUP | EPOLLWAKEUP | EPOLLET | EPOLLEXCLUSIVE;

/// EPOLLEXCLUSIVE as Linux allows it, EINVAL otherwise: never on MOD, nor
/// on an entry added with it, nor for a target that is itself an epoll, nor
/// beside a bit other than in, out, error, hang-up, wakeup and edge.
/// `present` is the events of the entry already there, if one is.
pub fn exclusive_ok(
    op: u64,
    events: u32,
    target_epoll: bool,
    present: Option<u32>,
) -> Result<(), i64> {
    let asks = events & EPOLLEXCLUSIVE != 0;
    let had = present.is_some_and(|e| e & EPOLLEXCLUSIVE != 0);
    let refused = match op {
        EPOLL_CTL_ADD => asks && (target_epoll || events & !EXCLUSIVE_OK != 0),
        EPOLL_CTL_MOD => asks || had,
        _ => false,
    };
    match refused {
        true => Err(errno::EINVAL),
        false => Ok(()),
    }
}
