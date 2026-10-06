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

use core::sync::atomic::{AtomicU32, Ordering};

use crate::state::globals::NET;
use crate::state::types::NetState;

/// Counts stores, so a handle kept across a serve-loop pass (a DNS lookup that
/// is still waiting) can tell the socket set it names was replaced.
static GENERATION: AtomicU32 = AtomicU32::new(0);

/// Which socket set is current.
pub fn generation() -> u32 {
    GENERATION.load(Ordering::Acquire)
}

pub fn store(state: NetState) {
    super::orphans::forget_all();
    // Every client's connections and ports name sockets in the old set too.
    // Their next call is told the socket is gone, which it is.
    crate::handles::forget_all();
    crate::udp_ports::forget_all();
    let mut net = NET.lock();
    GENERATION.fetch_add(1, Ordering::AcqRel);
    *net = Some(state);
}
