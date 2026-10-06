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

/* A model's size on the volume, importing a pinned one that is not there. */

use crate::linux::abi::errno;
use nonos_libc::{mk_data_import, mk_data_stat};

use super::pinned::pin_of;
use super::verified::{held_to, verified};

/*
 * The file's size. A pinned model not on the volume yet is imported first,
 * and kept only if it hashes to its pin; the outcome is said by name. One
 * that is there already is opened only if the kernel's import record of it
 * is its pin, digest and length.
 */
pub fn size_of(name: &[u8]) -> Result<u64, i64> {
    let got = mk_data_stat(name);
    let pin = match pin_of(name) {
        Some(pin) if got >= 0 => return verified(pin),
        Some(pin) if got == -errno::ENOENT => pin,
        None if got >= 0 => return Ok(got as u64),
        /* The volume itself refused: not ready, locked, or none. The person
         * at the terminal is told which, as after an import, not left with
         * the program's bare failure to open its model. */
        Some(pin) => {
            super::first_use::after(got, pin.tier);
            return Err(-got);
        }
        None => return Err(-got),
    };
    // Minutes for a large model: the person at a terminal is told why.
    super::first_use::before(name, pin.bytes);
    let done = mk_data_import(name, &pin.sha256, pin.bytes);
    super::first_use::after(done, pin.tier);
    let line = match done {
        n if n >= 0 => alloc::format!("[LINUX] model imported and verified: {n} bytes\n"),
        e => alloc::format!("[LINUX] model import refused, errno {}\n", -e),
    };
    if done < 0 {
        crate::linux::start::say(line.as_bytes());
    } else {
        crate::linux::start::routine(line.as_bytes());
    }
    if done < 0 {
        Err(-done)
    } else {
        held_to(pin, done)
    }
}
