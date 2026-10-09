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

//! epoll's rules, apart from any descriptor: what one look at an entry
//! reports, which event masks EPOLL_CTL refuses, and how far one epoll may
//! watch another. Pure, so the host proofs hold each of them.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::EPOLLET;

pub const EPOLL_CTL_ADD: u64 = 1;
pub const EPOLL_CTL_MOD: u64 = 3;
const EPOLLIN: u32 = 0x001;
const EPOLLOUT: u32 = 0x004;
const EPOLLERR: u32 = 0x008;
const EPOLLHUP: u32 = 0x010;
const EPOLLEXCLUSIVE: u32 = 1 << 28;
const EPOLLWAKEUP: u32 = 1 << 29;
/// The bits Linux allows beside EPOLLEXCLUSIVE.
const EXCLUSIVE_OK: u32 =
    EPOLLIN | EPOLLOUT | EPOLLERR | EPOLLHUP | EPOLLWAKEUP | EPOLLET | EPOLLEXCLUSIVE;
/// Linux's EP_MAX_NESTS: how deep one epoll may watch another.
pub const MAX_NESTS: u32 = 4;

/// What an entry sees of its descriptor's readiness `level`: what it asked
/// for, and hang-up and error, which are reported whether asked for or not.
pub fn seen(level: u32, events: u32) -> u32 {
    level & (events | EPOLLHUP | EPOLLERR)
}

/// What an entry reports: what it sees, and under EPOLLET only what rose
/// since `fired`, what it saw at the last look.
pub fn live(level: u32, events: u32, fired: u32) -> u32 {
    match events & EPOLLET {
        0 => seen(level, events),
        _ => seen(level, events) & !fired,
    }
}

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

/// Whether adding the epoll `target` to the epoll `ep` would make a loop, or
/// a chain of epolls deeper than MAX_NESTS: Linux answers ELOOP for either.
/// `lists` gives an epoll's watched descriptors, and None for a descriptor
/// that is not an epoll. Each epoll is walked once, as Linux marks the ones
/// it has seen, so no arrangement of epolls makes the walk long.
pub fn loops(lists: &impl Fn(u64) -> Option<Vec<u64>>, ep: u64, target: u64) -> bool {
    let mut seen: Vec<u64> = Vec::new();
    let mut next: Vec<(u64, u32)> = alloc::vec![(target, 0)];
    while let Some((at, depth)) = next.pop() {
        if at == ep || depth > MAX_NESTS {
            return true;
        }
        if seen.contains(&at) {
            continue;
        }
        seen.push(at);
        for fd in lists(at).unwrap_or_default() {
            if lists(fd).is_some() {
                next.push((fd, depth + 1));
            }
        }
    }
    false
}
