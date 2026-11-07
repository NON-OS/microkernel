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

//! `nslookup HOST`: the IPv4 address a host name has, asked of the
//! machine's resolver, and the name rules `ping` shares. A lookup leaves in
//! the clear, so the host answers it only on the Direct route and says why
//! not otherwise, as the terminal's `nslookup` does.

use alloc::vec::Vec;

use crate::host::{Host, Resolve};
use crate::out::ShellOut;
use crate::util::format_u64;

/// The longest name DNS carries.
pub const MAX_HOST: usize = 253;
/// Said, after the command's name, of a name `is_host` refuses.
pub const HOST_RULE: &[u8] = b": a host name is 1 to 253 letters, digits, dots and hyphens";
/// What a lookup off Direct would have done.
pub const IN_THE_CLEAR: &[u8] = b"a lookup would leave in the clear";

pub fn nslookup(out: &mut dyn ShellOut, host: &mut dyn Host, w: &[&[u8]]) -> bool {
    let [_, name] = w else {
        out.writeln(b"usage: nslookup <host>");
        return false;
    };
    if !is_host(name) {
        out.writeln(&[b"nslookup".as_slice(), HOST_RULE].concat());
        return false;
    }
    match host.resolve(name) {
        Resolve::Addr(ip) => {
            let mut line = name.to_vec();
            line.extend_from_slice(b" -> ");
            push_ip(&mut line, &ip);
            out.writeln(&line);
            true
        }
        other => {
            out.writeln(&failed(b"nslookup", name, other));
            false
        }
    }
}

/// The line a lookup that gave no address prints, under `command`.
pub fn failed(command: &[u8], name: &[u8], r: Resolve) -> Vec<u8> {
    let mut line = command.to_vec();
    line.extend_from_slice(b": ");
    match r {
        Resolve::Refused(why) => {
            line.extend_from_slice(why.as_bytes());
            line.extend_from_slice(b"; ");
            line.extend_from_slice(IN_THE_CLEAR);
            line.extend_from_slice(b", so nothing was sent");
        }
        Resolve::NoName => {
            line.extend_from_slice(b"no address for ");
            line.extend_from_slice(name);
        }
        Resolve::NoDns => line.extend_from_slice(b"no DNS server answered"),
        Resolve::Unavailable | Resolve::Addr(_) => line.extend_from_slice(b"dns unavailable"),
    }
    line
}

/// A name DNS could carry: 1 to MAX_HOST bytes of letters, digits, dots and
/// hyphens, so nothing else reaches the resolver.
pub fn is_host(name: &[u8]) -> bool {
    (1..=MAX_HOST).contains(&name.len())
        && name.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'.' || *c == b'-')
}

/// `a.b.c.d` as four bytes, or None.
pub fn parse_ipv4(s: &[u8]) -> Option<[u8; 4]> {
    let mut ip = [0u8; 4];
    let mut parts = s.split(|c| *c == b'.');
    for byte in ip.iter_mut() {
        let part = parts.next()?;
        if part.is_empty() || part.len() > 3 || !part.iter().all(u8::is_ascii_digit) {
            return None;
        }
        let v = part.iter().fold(0u16, |v, d| v * 10 + u16::from(d - b'0'));
        *byte = u8::try_from(v).ok()?;
    }
    parts.next().is_none().then_some(ip)
}

/// Append `a.b.c.d`.
pub fn push_ip(line: &mut Vec<u8>, ip: &[u8; 4]) {
    for (i, b) in ip.iter().enumerate() {
        if i > 0 {
            line.push(b'.');
        }
        let mut num = [0u8; 20];
        let n = format_u64(u64::from(*b), &mut num);
        line.extend_from_slice(&num[..n]);
    }
}
