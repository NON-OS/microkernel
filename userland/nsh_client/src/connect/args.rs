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

//! `nsh`'s arguments, `--name value` pairs. `--authority`, `--revoked-enrol`
//! and `--escalate` may repeat; every other name is given once.

pub use super::hex::hex;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Args {
    pub connect: String,
    pub host: String,
    pub fleet: [u8; 16],
    pub authorities: Vec<String>,
    pub releases: String,
    pub key: String,
    pub grant: String,
    pub revoked: Vec<String>,
    pub escalate: Vec<String>,
    pub records: Option<String>,
    /// Seconds the host has to admit the login.
    pub handshake: u64,
    /// Seconds the host has, once the session is closed, to end it and send
    /// its record.
    pub close_wait: u64,
}

pub const USAGE: &str = "usage: nsh --connect HOST:PORT --host NAME --fleet HEX --authority FILE \
[--authority FILE ...] --releases FILE --key FILE --grant FILE [--revoked-enrol FILE ...] \
[--escalate FILE ...] [--records DIR] [--handshake SECS] [--close-wait SECS]";

/// Seconds a host has to admit the login unless --handshake says otherwise,
/// and the most --handshake takes.
pub const HANDSHAKE_SECS: u64 = 60;
pub const MAX_HANDSHAKE_SECS: u64 = 3600;
/// Seconds a host has to end a closed session unless --close-wait says
/// otherwise: the minute nsh.front gives its shell to exit after a stop.
/// --close-wait takes up to MAX_HANDSHAKE_SECS.
pub const CLOSE_WAIT_SECS: u64 = 60;

pub fn parse(args: &[String]) -> Option<Args> {
    let mut one: Vec<(&str, &String)> = Vec::new();
    let (mut authorities, mut revoked, mut escalate) = (Vec::new(), Vec::new(), Vec::new());
    let mut rest = args;
    while let [name, value, tail @ ..] = rest {
        match name.as_str() {
            "--authority" => authorities.push(value.clone()),
            "--revoked-enrol" => revoked.push(value.clone()),
            "--escalate" => escalate.push(value.clone()),
            n if one.iter().any(|(k, _)| *k == n) => return None,
            n => one.push((n, value)),
        }
        rest = tail;
    }
    let get = |n: &str| one.iter().find(|(k, _)| *k == n).map(|(_, v)| (*v).clone());
    let known = [
        "--connect",
        "--host",
        "--fleet",
        "--releases",
        "--key",
        "--grant",
        "--records",
        "--handshake",
        "--close-wait",
    ];
    if !rest.is_empty() || authorities.is_empty() || one.iter().any(|(k, _)| !known.contains(k)) {
        return None;
    }
    Some(Args {
        connect: get("--connect")?,
        host: get("--host")?.to_ascii_lowercase(),
        fleet: hex(&get("--fleet")?)?,
        authorities,
        releases: get("--releases")?,
        key: get("--key")?,
        grant: get("--grant")?,
        revoked,
        escalate,
        records: get("--records"),
        handshake: secs(get("--handshake"), HANDSHAKE_SECS)?,
        close_wait: secs(get("--close-wait"), CLOSE_WAIT_SECS)?,
    })
}

/// A wait in seconds, 1 to MAX_HANDSHAKE_SECS, or `default` when not given.
fn secs(given: Option<String>, default: u64) -> Option<u64> {
    match given {
        None => Some(default),
        Some(t) => t.parse().ok().filter(|t| (1..=MAX_HANDSHAKE_SECS).contains(t)),
    }
}
