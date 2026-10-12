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

//! The bus kept after bring-up: offers the host adds are kept and named,
//! and an offer it withdraws is forgotten and released. No channel is
//! opened here; each waits for a driver.

use nonos_libc::mk_idle_ms;

use crate::connect::Bus;
use crate::constants::{MESSAGES_PER_LOOK, OFFERCHANNEL, POLL_MS, RESCIND_CHANNELOFFER};
use crate::hv::Hv;
use crate::msg::{self, kind, relid_released};
use crate::offers::Offers;
use crate::say;
use crate::setup::Driver;

pub fn run(d: &mut Driver) -> ! {
    loop {
        if step(d) == 0 {
            let _ = mk_idle_ms(POLL_MS);
        }
    }
}

/// One look: the messages handled, at most MESSAGES_PER_LOOK.
pub fn step(d: &mut Driver) -> usize {
    let mut n = 0;
    while n < MESSAGES_PER_LOOK {
        let Some(m) = d.hv.recv() else { break };
        n += 1;
        match kind(m.bytes()) {
            Some(OFFERCHANNEL) => {
                if let Some(o) = msg::offer(m.bytes()) {
                    if d.offers.insert(o) {
                        say::offered(&o);
                    } else {
                        say::full(&o);
                    }
                }
            }
            Some(RESCIND_CHANNELOFFER) => {
                let _ = withdraw(&d.hv, d.bus, &mut d.offers, m.bytes());
            }
            _ => {}
        }
    }
    n
}

/// A withdrawal: the offer forgotten, and the relid given back, which the
/// host waits for before it offers the slot again.
pub fn withdraw(hv: &Hv, bus: Bus, offers: &mut Offers, p: &[u8]) -> Result<(), &'static str> {
    let relid = msg::rescind(p).ok_or("withdrawal cut short")?;
    if let Some(o) = offers.remove(relid) {
        say::withdrawn(&o);
    }
    hv.post(bus.connection, &relid_released(relid))
}
