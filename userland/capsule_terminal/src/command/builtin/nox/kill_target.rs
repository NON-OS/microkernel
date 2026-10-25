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

#[path = "kill_target_refused.rs"]
mod kill_target_refused;
#[path = "kill_target_resolve.rs"]
mod kill_target_resolve;

pub use kill_target_refused::refused;
pub use kill_target_resolve::resolve;

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
