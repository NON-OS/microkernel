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

/*
 * A pinned model on the volume, held to its pin before it is opened. Its
 * size says nothing: anything that writes the volume could have put a file
 * under the name. The kernel's import record does: asked to import a file
 * it already verified, the kernel answers from the record it sealed beside
 * it, without hashing again, with the file's length when that record is
 * the pinned SHA-256, and EEXIST when it is another digest or there is none.
 */

use alloc::format;

use nonos_libc::mk_data_import;

use super::pinned::Pinned;

const EBADMSG: i64 = 74;

/* The model's length when the kernel's record of it is `pin`, whole. */
pub(super) fn verified(pin: &Pinned) -> Result<u64, i64> {
    held_to(pin, mk_data_import(pin.name, &pin.sha256, pin.bytes))
}

/* `done`, an import's answer for `pin`, as the model's length or its refusal. */
pub(super) fn held_to(pin: &Pinned, done: i64) -> Result<u64, i64> {
    if done >= 0 && done as u64 == pin.bytes {
        return Ok(pin.bytes);
    }
    let file = core::str::from_utf8(&pin.name[1..]).unwrap_or("model");
    let why = match done {
        n if n >= 0 => format!("{n} bytes on the volume, {} pinned", pin.bytes),
        e => format!("errno {}", -e),
    };
    let line = format!("refused model {file}: its import record is not the signed pin ({why})\n");
    crate::linux::start::say(format!("[LINUX] {line}").as_bytes());
    crate::linux::console::say(format!("qwen: {line}").as_bytes());
    Err(if done < 0 { -done } else { EBADMSG })
}
