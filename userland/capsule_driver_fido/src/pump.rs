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
//! The key's IN endpoint read while a request runs: keep-alives keep it
//! waiting, its answer or error ends it. A key quiet too long has its
//! request cancelled and the channel counted spent.

use nonos_libc::Deadline;
use nonos_usbnet::Bus;

use crate::ctaphid::PACKET;
use crate::device::{Dev, QUIET_MS};
use crate::request::spent;

/// Reports read a look: a 4000-byte answer is 68 of them.
const LOOKS: usize = 16;

pub fn pump(d: &mut Dev) {
    if !d.job.busy() {
        return;
    }
    let (Some(mut bus), Some(dci)) = (d.bus(), d.port().map(|p| p.dci.0)) else { return };
    let mut p = [0u8; PACKET];
    for _ in 0..LOOKS {
        let Ok(Some(n)) = bus.interrupt_in_poll(dci, &mut p) else { break };
        if d.job.hear(&p[..n.min(PACKET)]) {
            d.quiet = Deadline::after_ms(QUIET_MS);
        }
    }
    if d.quiet.expired() {
        let ran = d.job.give_up();
        spent(d, ran);
    }
}
