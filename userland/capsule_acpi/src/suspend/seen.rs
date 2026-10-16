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

//! One boot line saying what this machine offers for sleep, so the owner's
//! S0-idle decision rests on facts: whether \_S3 exists, whether S0
//! low-power idle is offered (the FADT's flag, or an LPS0 _DSM holding a
//! known UUID), what LPIT lists, and that NONOS never enters S0 idle. Read
//! only: nothing is entered and no _DSM is called.

use super::lpit::states;
use crate::boot::Acpi;
use crate::sys::log::Line;
use nonos_aml::suspend::{fadt_s0_idle, has_s3, lps0_seen, offered, Lps0Seen};

fn yes(b: bool) -> &'static [u8] {
    if b {
        b"yes"
    } else {
        b"no"
    }
}

pub fn say(acpi: &Acpi, fadt: &[u8]) {
    let (flag, lps0) = (fadt_s0_idle(fadt), lps0_seen(&acpi.aml, &acpi.devices));
    let l = Line::new(b"sleep: S3 ").str(yes(has_s3(&acpi.aml)));
    let l = l.str(b", S0 idle offered ").str(yes(offered(flag, lps0)));
    let l = l.str(b" (FADT flag ").str(yes(flag)).str(b", LPIT ");
    let l = l.dec(states().len() as u64).str(b" states, ");
    let l = match lps0 {
        Lps0Seen::Absent => l.str(b"no LPS0 device"),
        Lps0Seen::NoDsm => l.str(b"LPS0 device without _DSM"),
        Lps0Seen::Dsm(u) if !(u.microsoft || u.intel || u.amd) => {
            l.str(b"LPS0 _DSM, no known UUID")
        }
        Lps0Seen::Dsm(u) => {
            let l = l.str(b"LPS0 _DSM");
            let l = if u.microsoft { l.str(b" microsoft") } else { l };
            let l = if u.intel { l.str(b" intel") } else { l };
            if u.amd {
                l.str(b" amd")
            } else {
                l
            }
        }
    };
    l.str(b"), entered never").end();
}
