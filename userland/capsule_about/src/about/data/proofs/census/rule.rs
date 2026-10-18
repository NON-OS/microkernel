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

//! The census itself: every live process, admitted or named for why not.

use super::super::firmware::is_firmware;
use super::counts::Census;
use super::marks::{Admitted, Proc, AUTHORITY_PUBLISHER, CAP_NETWORK, STARTING_MS};

const STATE_NEW: u8 = 0;

const STATE_ZOMBIE: u8 = 5;

const INIT: &[u8] = b"init";

pub fn census(procs: &[Proc<'_>], admitted: &[Admitted]) -> Census {
    let mut c = Census::default();
    for p in procs {
        if p.state == STATE_NEW || p.state == STATE_ZOMBIE {
            continue;
        }
        c.running += 1;
        // The firmware entry shares pid 0 with no program; it admits nothing.
        if let Some(a) = admitted.iter().find(|a| a.pid == p.pid && !is_firmware(a.pid)) {
            c.admitted += 1;
            match a.authority {
                0 => c.vendor += 1,
                AUTHORITY_PUBLISHER => c.signed_only += 1,
                _ => c.enrolled += 1,
            }
            if a.caps & CAP_NETWORK != 0 {
                c.network += 1;
            }
        } else if p.name == INIT {
            c.kernel += 1;
        } else if p.caps == 0 {
            c.sandboxed += 1;
        } else if p.uptime_ms < STARTING_MS {
            c.starting += 1;
        } else {
            c.unadmitted += 1;
            if c.first_unadmitted.is_none() {
                c.first_unadmitted = Some(p.pid);
            }
        }
    }
    c
}
