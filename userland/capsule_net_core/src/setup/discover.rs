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

//! One discovery pass over the candidates.

use core::sync::atomic::{AtomicUsize, Ordering};

use nonos_libc::mk_service_lookup;

use super::candidates::{candidate, count};
use super::log::probe_seen;
use crate::device;

/// How many NIC drivers one discovery pass may block on.
const MAX_PROBES_PER_PASS: usize = 2;

/// Where the next discovery pass resumes, so a long candidate list is swept
/// across passes instead of inside one.
static PROBE_CURSOR: AtomicUsize = AtomicUsize::new(0);

// Return the first candidate whose link is actually up. Driver capsules register
// their service name at spawn, before probing hardware, so a wired NIC with no
// chip present or no cable attached still resolves through the service lookup.
// Binding the first name that merely exists stranded net_core on that dead
// interface: it reported link-down forever and net_core never fell through to the
// WiFi link, which only comes up once the user associates. Requiring an up link
// means net_core waits during boot and binds the WiFi interface the moment it
// connects, while still preferring a wired NIC that genuinely has carrier. A
// racing, dying wired capsule answers link_up with `None`, which is treated as
// not-up and skipped rather than taking net_core down.
// Each probe is a driver round trip the serve loop is blocked on, so a pass
// spends at most MAX_PROBES_PER_PASS of them and resumes at the next candidate
// a second later. Every real configuration registers one or two NIC drivers and
// so still sweeps the whole list in a single pass.
pub(super) fn discover_nic() -> Option<(u32, &'static str)> {
    let count = count();
    let start = PROBE_CURSOR.load(Ordering::Relaxed);
    let mut probes = 0usize;
    for step in 0..count {
        let idx = (start + step) % count;
        let name = candidate(idx);
        let mut port: u32 = 0;
        let mut pid: u32 = 0;
        if mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid) != 0 {
            continue;
        }
        probes += 1;
        match device::link_up(port) {
            Some(true) => {
                PROBE_CURSOR.store(idx, Ordering::Relaxed);
                return Some((port, name));
            }
            verdict => probe_seen(idx, name, verdict),
        }
        if probes == MAX_PROBES_PER_PASS {
            PROBE_CURSOR.store((idx + 1) % count, Ordering::Relaxed);
            return None;
        }
    }
    PROBE_CURSOR.store(0, Ordering::Relaxed);
    None
}
