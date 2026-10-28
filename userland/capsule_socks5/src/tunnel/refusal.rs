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

//! The class of an exit's refusal, read from a connection error's message
//! (decode.rs): enough for a trace to say why a stream did not open without
//! saying where it was going. The message names the host, so only the class
//! leaves here: a dead name read as "exit answered bytes 67" until it was
//! looked up by hand. Pure.

use super::constants::{PROTOCOL_VERSION, RESP_CONNECTION_ERROR};
use super::provider::inner_response;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refusal {
    Dns,
    Policy,
    Timeout,
    Refused,
    Unreachable,
    Other,
}

/// The words that name each class, in the order they are tried.
const CLASSES: &[(Refusal, &[&[u8]])] = &[
    (Refusal::Dns, &[b"resolve", b"lookup", b"dns", b"name or service", b"nodename"]),
    (Refusal::Policy, &[b"policy", b"not allowed", b"allowed list", b"allow list"]),
    (Refusal::Timeout, &[b"timed out", b"timeout"]),
    (Refusal::Refused, &[b"refused"]),
    (Refusal::Unreachable, &[b"unreachable"]),
];

impl Refusal {
    pub fn word(self) -> &'static [u8] {
        match self {
            Refusal::Dns => b"dns",
            Refusal::Policy => b"policy",
            Refusal::Timeout => b"timeout",
            Refusal::Refused => b"refused",
            Refusal::Unreachable => b"unreachable",
            Refusal::Other => b"other",
        }
    }
}

/// The class of the connection error `buf` carries: version, kind, the
/// connection id's 8 bytes, then the message. None for any other reply.
pub fn refusal(buf: &[u8]) -> Option<Refusal> {
    let raw = inner_response(buf)?;
    let message =
        raw.get(10..).filter(|_| raw[..2] == [PROTOCOL_VERSION, RESP_CONNECTION_ERROR])?;
    let has = |w: &[u8]| message.windows(w.len()).any(|m| m.eq_ignore_ascii_case(w));
    let found = CLASSES.iter().find(|(_, words)| words.iter().any(|w| has(w)));
    Some(found.map_or(Refusal::Other, |(class, _)| *class))
}
