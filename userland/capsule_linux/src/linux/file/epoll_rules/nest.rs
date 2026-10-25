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

//! How far one epoll may watch another. Pure, so the host proofs hold it.

use alloc::vec::Vec;

/// Linux's EP_MAX_NESTS: how deep one epoll may watch another.
pub const MAX_NESTS: u32 = 4;

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
