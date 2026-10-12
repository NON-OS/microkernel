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

//! The capsule's loop: a request when one comes, the SCI between them. The
//! receive waits a few milliseconds at most, so an SCI is served within
//! that of its arrival even when no one asks anything.

use super::protocol::MAX;
use super::reply::answer;
use super::sci::serve;
use crate::boot::Acpi;
use alloc::vec;
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, recv_ready};

const SERVICE_PORT: u64 = 4530;
const RECV_MS: u64 = 10;

pub fn run(mut acpi: Acpi) -> ! {
    let mut buf = vec![0u8; MAX];
    loop {
        let mut sender = 0u32;
        let got = mk_ipc_recv_from(SERVICE_PORT, buf.as_mut_ptr(), buf.len(), RECV_MS, &mut sender);
        if recv_ready(got) && sender != 0 {
            let out = answer(&mut acpi, &buf[..got as usize], sender);
            let _ = mk_ipc_reply(sender, out.as_ptr(), out.len());
        }
        serve(&mut acpi);
        crate::power::tick(&mut acpi.power, &mut acpi.aml, &mut acpi.host);
        crate::cooling::tick(&mut acpi.cooling, &mut acpi.aml, &mut acpi.host);
        crate::dpower::tick(&mut acpi);
    }
}
