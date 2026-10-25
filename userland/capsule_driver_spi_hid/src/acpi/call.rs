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

//! The firmware's serial-bus records asked of capsule_acpi one index at a
//! time, until one is a HID-over-SPI device on spi0's bus this driver can
//! run.

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use super::parse::parse;
use super::pick::{choose, Choice};
use super::wire::{request, Serial, REPLY_MAX};

const SERVICE: &[u8] = b"acpi";
const CALL_MS: u64 = 200;
/// Records looked at, whatever count the first says.
const RECORDS_MAX: u32 = 64;
const E_PERM: i32 = -1;

pub fn find(host_key: u64) -> Result<Choice, &'static str> {
    let port = lookup().ok_or("spi_hid: capsule_acpi is not there")?;
    let mut why = "spi_hid: the firmware lists no serial-bus device";
    let (mut index, mut count) = (0, 1);
    while index < count.min(RECORDS_MAX) {
        let s = match serial(port, index) {
            Some(Ok(s)) => s,
            Some(Err(E_PERM)) => return Err("spi_hid: capsule_acpi refuses this driver"),
            _ => break,
        };
        match choose(&s, host_key) {
            Ok(c) => return Ok(c),
            Err(e) => why = e,
        }
        (index, count) = (index + 1, s.count);
    }
    Err(why)
}

fn serial(port: u64, index: u32) -> Option<Result<Serial, i32>> {
    let req = request(index);
    let mut reply = [0u8; REPLY_MAX];
    let (rp, rl) = (reply.as_mut_ptr(), reply.len());
    let n = mk_ipc_call_timeout(port, req.as_ptr(), req.len(), rp, rl, CALL_MS);
    parse(&reply[..usize::try_from(n).ok()?.min(REPLY_MAX)])
}

fn lookup() -> Option<u64> {
    let (mut port, mut pid) = (0u32, 0u32);
    let r = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    (r >= 0 && port != 0 && pid != 0).then_some(port as u64)
}
