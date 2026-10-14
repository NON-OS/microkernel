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

//! Which RPC host each network asks now, and moving on from one that failed.

use core::sync::atomic::{AtomicUsize, Ordering};

use super::{current, is_sepolia};

/* Which of each network's hosts is asked now, by its place in `rpcs`. */
static HOST: [AtomicUsize; 2] = [AtomicUsize::new(0), AtomicUsize::new(0)];

fn slot() -> &'static AtomicUsize {
    &HOST[usize::from(is_sepolia())]
}

/// The RPC host the picked network's requests go to now.
pub fn rpc_host() -> &'static str {
    let hosts = current().rpcs;
    hosts.get(slot().load(Ordering::Relaxed) % hosts.len().max(1)).copied().unwrap_or("")
}

/// `host` failed a request: the next one is asked from now on. Only when it
/// is still the one asked, so two requests that failed on it move one place.
/// True when the host changed.
pub fn rpc_failed(host: &str) -> bool {
    let hosts = current().rpcs;
    let at = slot().load(Ordering::Relaxed);
    if hosts.len() < 2 || hosts.get(at % hosts.len()) != Some(&host) {
        return false;
    }
    slot()
        .compare_exchange(at, (at + 1) % hosts.len(), Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
}
