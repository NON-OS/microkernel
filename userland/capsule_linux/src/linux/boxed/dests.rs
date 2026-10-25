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

//! The destinations an `allow:` box names, as the canonical list the
//! personality already holds streams to (nonos_linux_policy::dests): one port
//! each, an IPv4 address whole or a name, sorted and once.

use alloc::vec::Vec;

use nonos_linux_policy::dests::{check, MAX_ENTRIES};

/// The list for `items` (`<address or name>:<port>`), or empty for none.
pub fn list(items: &[&[u8]]) -> Result<Vec<u8>, &'static str> {
    let mut entries = items.iter().map(|i| entry(i)).collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    entries.dedup();
    if entries.is_empty() {
        return Ok(Vec::new());
    }
    if entries.len() > MAX_ENTRIES {
        return Err("more than 64 destinations");
    }
    let mut out = (entries.len() as u16).to_be_bytes().to_vec();
    entries.iter().for_each(|e| out.extend_from_slice(e));
    check(&out).map_err(|_| "a destination is not an address or name with a port")?;
    Ok(out)
}

fn entry(item: &[u8]) -> Result<Vec<u8>, &'static str> {
    let at = item.iter().rposition(|b| *b == b':').ok_or("a destination names its port")?;
    let (host, port) = (&item[..at], &item[at + 1..]);
    let port = decimal(port).filter(|p| *p != 0).ok_or("a port is 1 to 65535")?;
    let (kind, prefix, body) = match ipv4(host) {
        Some(ip) => (4u8, 32u8, ip.to_vec()),
        None => (b'N', 0u8, host.to_ascii_lowercase()),
    };
    let len = u8::try_from(body.len()).map_err(|_| "a name is at most 253 bytes")?;
    let mut out = alloc::vec![kind, prefix];
    out.extend_from_slice(&port.to_be_bytes());
    out.extend_from_slice(&port.to_be_bytes());
    out.push(len);
    out.extend_from_slice(&body);
    Ok(out)
}

fn ipv4(host: &[u8]) -> Option<[u8; 4]> {
    let mut out = [0u8; 4];
    let mut parts = host.split(|b| *b == b'.');
    for o in out.iter_mut() {
        *o = u8::try_from(decimal(parts.next()?)?).ok()?;
    }
    parts.next().is_none().then_some(out)
}

fn decimal(digits: &[u8]) -> Option<u16> {
    if digits.is_empty() || digits.len() > 5 || !digits.iter().all(u8::is_ascii_digit) {
        return None;
    }
    digits.iter().try_fold(0u16, |n, d| n.checked_mul(10)?.checked_add(u16::from(d - b'0')))
}
