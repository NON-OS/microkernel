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

//! Whether a guest's box grants a service, and the refusal when it does
//! not, said by name where the person running it reads it.

use nonos_linux_policy::dests::Target;
use nonos_linux_policy::mask::grants;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

/// Whether `guest` is served `bit` (nonos_linux_policy::mask).
pub fn granted(guest: &Guest, bit: u64) -> bool {
    grants(guest.app.mask, bit)
}

/// An internet socket (rules::inet_allowed).
pub fn inet(guest: &Guest) -> bool {
    super::rules::inet_allowed(guest.app.mask)
}

/// A stream out of the family to `to` on `port` (reach::may_dial).
pub fn may_dial(guest: &Guest, to: Target<'_>, port: u16) -> bool {
    super::reach::may_dial(guest.app.mask, &guest.app.reach, to, port)
}

/// `errno` for `what`, after saying that the program's policy does not
/// grant `needs`.
pub fn refuse(what: &str, needs: &str, code: i64) -> u64 {
    crate::linux::boxed::note(crate::linux::boxed::of_needs(needs), what.as_bytes());
    // `what` may carry a name the guest chose: shown, never obeyed as a control.
    let what: alloc::string::String =
        what.chars().map(|c| if c.is_ascii_graphic() || c == ' ' { c } else { '?' }).collect();
    let line = alloc::format!("[LINUX] refused {what}: its policy does not grant {needs}\n");
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
    errno::fail(code)
}
