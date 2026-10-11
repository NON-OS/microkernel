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

//! A vendor program's destinations as policies.txt writes them after
//! `dests=`: items joined by commas, each `<IPv4>/<prefix>:<ports>`,
//! `[<IPv6>]/<prefix>:<ports>` or `<name>:<ports>`, the ports one number or
//! `<first>-<last>`. They are written out in the canonical form (src/dests),
//! sorted, and a list that form refuses fails the build.

use std::net::{Ipv4Addr, Ipv6Addr};

use crate::dests::check;

pub fn dests_of(text: &str) -> Result<Vec<u8>, String> {
    let mut entries = text.split(',').map(entry).collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    let count = u16::try_from(entries.len()).map_err(|_| format!("dests={text}: too many"))?;
    let mut list = count.to_be_bytes().to_vec();
    entries.iter().for_each(|e| list.extend_from_slice(e));
    check(&list).map(|_| list).map_err(|why| format!("dests={text}: {why:?}"))
}

fn entry(item: &str) -> Result<Vec<u8>, String> {
    let bad = |what: &str| format!("dests item {item:?}: {what}");
    let (what, ports) = item.rsplit_once(':').ok_or_else(|| bad("no ports"))?;
    let (first, last) = ports_of(ports).ok_or_else(|| bad("not a port or range"))?;
    let (kind, prefix, body) = match what.split_once('/') {
        None => (b'N', 0, what.as_bytes().to_vec()),
        Some((addr, prefix)) => {
            let prefix: u8 = prefix.parse().map_err(|_| bad("not a prefix length"))?;
            match addr.strip_prefix('[').and_then(|a| a.strip_suffix(']')) {
                Some(v6) => {
                    (6, prefix, v6.parse::<Ipv6Addr>().map_err(|_| bad("IPv6"))?.octets().to_vec())
                }
                None => (
                    4,
                    prefix,
                    addr.parse::<Ipv4Addr>().map_err(|_| bad("IPv4"))?.octets().to_vec(),
                ),
            }
        }
    };
    let len = u8::try_from(body.len()).map_err(|_| bad("too long"))?;
    let mut e = vec![kind, prefix];
    e.extend_from_slice(&first.to_be_bytes());
    e.extend_from_slice(&last.to_be_bytes());
    e.push(len);
    e.extend_from_slice(&body);
    Ok(e)
}

fn ports_of(text: &str) -> Option<(u16, u16)> {
    match text.split_once('-') {
        Some((first, last)) => Some((first.parse().ok()?, last.parse().ok()?)),
        None => text.parse().ok().map(|p| (p, p)),
    }
}
