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
//! Who a session belongs to, as whatever admitted it says, and `whoami`,
//! which prints it. The GUI terminal never sets it; nsh.front sets it from
//! the login it admitted, before the first line runs.

use crate::out::ShellOut;
use alloc::vec::Vec;

/// The login a session runs as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Who {
    pub user: Vec<u8>,
    pub role: Vec<u8>,
    /// SHA-512 of the user's public key.
    pub subject: [u8; 64],
    /// The capabilities the login's role grants, by name. A command that
    /// reaches past the shell (`builtin_cap`) runs only if its capability
    /// is here, whatever the host itself holds.
    pub caps: Vec<&'static str>,
}

/// The capability a builtin needs of the login's role, if any.
pub(super) fn builtin_cap(name: &[u8]) -> Option<&'static str> {
    match name {
        b"service" | b"svc" => Some("IPC"),
        b"ping" | b"nslookup" => Some("Network"),
        _ => None,
    }
}

/// Whether `name` may run for this login, having said why not. A session
/// nothing admitted (the GUI terminal's) runs under its host's own grant.
pub(super) fn role_allows(who: Option<&Who>, name: &[u8], out: &mut dyn ShellOut) -> bool {
    let (Some(who), Some(cap)) = (who, builtin_cap(name)) else { return true };
    if who.caps.contains(&cap) {
        return true;
    }
    out.writeln(
        &[
            &shown(name)[..],
            b": the role ",
            &shown(&who.role),
            b" does not hold ",
            cap.as_bytes(),
            b", which this command needs",
        ]
        .concat(),
    );
    false
}

/// Said by `whoami` in a session nothing admitted.
pub const NO_LOGIN: &[u8] = b"whoami: this session has no login";

/// `whoami`: the user, the role and the key's digest, one line each.
pub(super) fn whoami(who: Option<&Who>, out: &mut dyn ShellOut) -> bool {
    let Some(who) = who else {
        out.writeln(NO_LOGIN);
        return false;
    };
    out.writeln(&[&b"user "[..], &shown(&who.user)].concat());
    out.writeln(&[&b"role "[..], &shown(&who.role)].concat());
    let mut key = Vec::with_capacity(11 + 128);
    key.extend_from_slice(b"key sha512 ");
    for b in who.subject {
        key.push(HEX[usize::from(b >> 4)]);
        key.push(HEX[usize::from(b & 0x0f)]);
    }
    out.writeln(&key);
    true
}

const HEX: &[u8; 16] = b"0123456789abcdef";

/// A name as printable ASCII: any other byte is shown as `?`, so no name
/// can reach the peer's terminal as a control sequence.
fn shown(name: &[u8]) -> Vec<u8> {
    name.iter().map(|&c| if (0x20..0x7f).contains(&c) { c } else { b'?' }).collect()
}
