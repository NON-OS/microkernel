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

use smoltcp::iface::{Interface, SocketSet};

use crate::state::globals::NET;
use crate::state::types::DnsSockets;

pub fn with_dns<R>(
    f: impl FnOnce(&mut Interface, &mut SocketSet<'static>, DnsSockets) -> R,
) -> Option<R> {
    let mut guard = NET.lock();
    let state = guard.as_mut()?;
    if state.dns.iter().all(Option::is_none) {
        return None;
    }
    Some(f(&mut state.iface, &mut state.sockets, state.dns))
}

/// Run `f` on the DNS sockets `dns` names, only while they are still the ones
/// in use: in the socket set of `generation` and not replaced by a later
/// lease. A lookup kept across passes asks here, since its query handles mean
/// nothing to another socket (and smoltcp panics on a free slot).
pub fn with_dns_at<R>(
    generation: u32,
    dns: DnsSockets,
    f: impl FnOnce(&mut SocketSet<'static>) -> R,
) -> Option<R> {
    let mut guard = NET.lock();
    if super::store::generation() != generation {
        return None;
    }
    let state = guard.as_mut()?;
    if state.dns != dns {
        return None;
    }
    Some(f(&mut state.sockets))
}
