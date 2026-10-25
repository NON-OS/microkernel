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

//! Which station address the driver serves the stack, decided before the
//! link is ever reported up, so the stack reads the address once and it
//! never changes under it. The drawn address is offered to the host first
//! (Linux's rndis_filter_set_device_mac); a host that refuses it outright
//! leaves the host-assigned address at once. A host that takes it is then
//! held to it: the probe's DISCOVERs go out from the drawn address, and
//! only an answer to it keeps it. Nothing within the bounded wait, and the
//! host-assigned address is taken. Either fallback is said on the console.

use nonos_vmbus::{Host, Shared};

use crate::constants::timing::{PROBE_EVERY_MS, PROBE_TRIES, SEND_MS};
use crate::link::Link;
use crate::probe::{answers, discover, Probe, Step, Verdict};
use crate::start::offer_mac;

/// The verdict on the drawn address `drawn`, the probe's DHCP transaction
/// being `xid`.
pub fn choose<H: Host, M: Shared>(l: &mut Link<H, M>, drawn: &[u8; 6], xid: u32) -> Result<Verdict, &'static str> {
    if !offer_mac(l, drawn)? {
        return Ok(Verdict::Refused);
    }
    let mut p = Probe::new();
    loop {
        let now = l.host.now_ms();
        match p.step(now, PROBE_TRIES, PROBE_EVERY_MS) {
            Step::Send => {
                // A DISCOVER the ring could not take is tried again on the
                // next pass; it counts as sent so the wait stays bounded.
                let _ = l.send_frame(&discover(drawn, xid), SEND_MS);
                p.sent_at(now);
            }
            Step::GiveUp => return Ok(Verdict::Unanswered),
            Step::Listen => {}
        }
        l.service().map_err(|_| "channel broken during the probe")?;
        while let Some(f) = l.frames.pop_front() {
            if answers(&f, drawn, xid) {
                l.frames.clear();
                return Ok(Verdict::Drawn);
            }
        }
        l.host.idle();
    }
}

/// The address the verdict leaves the driver with.
pub const fn address(v: Verdict, drawn: [u8; 6], host: [u8; 6]) -> [u8; 6] {
    match v {
        Verdict::Drawn => drawn,
        Verdict::Refused | Verdict::Unanswered => host,
    }
}
