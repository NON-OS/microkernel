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

// What `kill` was given: a pid, or a name that the service registry answers
// for. A name is tried as typed, then as an app, a tool and a network
// service, so `kill browser` reaches app.browser the way pkill would.

/// The prefixes a bare name is tried under, in order.
pub const PREFIXES: [&[u8]; 4] = [b"", b"app.", b"tool.", b"net."];

/// The longest service name the registry holds.
pub const NAME_MAX: usize = 48;

#[derive(Debug, PartialEq, Eq)]
pub enum Target {
    Pid(u64),
    Name,
    /// Neither a number nor a name a service could carry.
    Unreadable,
}

pub fn target(arg: &[u8]) -> Target {
    if let Some(pid) = parse_u64(arg) {
        return Target::Pid(pid);
    }
    // All digits that did not parse is a pid too large for any process.
    let named = !arg.is_empty()
        && !arg.iter().all(u8::is_ascii_digit)
        && arg.len() <= NAME_MAX
        && arg.iter().all(|&b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'));
    if named {
        Target::Name
    } else {
        Target::Unreadable
    }
}

/// The first prefixed spelling of `name` that `lookup` finds a pid for: its
/// length in `buf`, which holds it, and the pid.
pub fn resolve(
    name: &[u8],
    buf: &mut [u8; NAME_MAX],
    mut lookup: impl FnMut(&[u8]) -> Option<u64>,
) -> Option<(usize, u64)> {
    for prefix in PREFIXES {
        let len = prefix.len() + name.len();
        if len > NAME_MAX {
            continue;
        }
        buf[..prefix.len()].copy_from_slice(prefix);
        buf[prefix.len()..len].copy_from_slice(name);
        if let Some(pid) = lookup(&buf[..len]) {
            return Some((len, pid));
        }
    }
    None
}

/// Why the kernel refused the kill, in words. The terminal holds no
/// ProcessControl, so it ends only the processes it started; anything else
/// is the Process Manager's to end.
pub fn refused(rc: i64) -> &'static [u8] {
    match rc {
        -1 => b"kill: the terminal ends only what it started; Process Manager can end this one",
        _ => b"kill: the signal must be 2, 9 or 15",
    }
}

pub fn parse_u64(a: &[u8]) -> Option<u64> {
    if a.is_empty() {
        return None;
    }
    let mut v: u64 = 0;
    for &b in a {
        if !b.is_ascii_digit() {
            return None;
        }
        v = v.checked_mul(10)?.checked_add((b - b'0') as u64)?;
    }
    Some(v)
}
