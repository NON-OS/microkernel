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
//! The slot-change endpoint looked at: each notice marks the slots it says
//! changed, for the holder's next STATUS. A card pulled from a slot this
//! driver powered is powered no longer, so CLOSE does not ask it again.

use nonos_usbnet::Bus;

use crate::device::Dev;

/// NotifySlotChange is 1 byte and 2 bits a slot; 64 bytes is 252 slots.
const NOTICE_MAX: usize = 64;

pub fn pump(d: &mut Dev) {
    let (Some(mut bus), Some(p)) = (d.bus(), d.port()) else { return };
    let (Some(dci), max_slot) = (p.notify, p.r.max_slot) else { return };
    let mut n = [0u8; NOTICE_MAX];
    if let Ok(Some(len)) = bus.interrupt_in_poll(dci, &mut n) {
        d.slots.seen(&n[..len.min(NOTICE_MAX)], max_slot);
    }
}
