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

//! Role grants, `nsh-grant/1`: the user's base grant, and an escalation
//! under it, each signed by the role authority.

use super::keys::{self, Hybrid};
use super::{put8, FLEET};
use nsh_crypto::{labelled, Label};

pub const GRANT_ID: [u8; 16] = [0x11; 16];
pub const CAPS: [&[u8]; 2] = [b"CoreExec", b"IPC"];

/// A grant of `kind` (1 base, 2 escalation) for `subject`'s keys.
pub fn grant(kind: u8, subject: &Hybrid) -> Vec<u8> {
    let mut out = b"NSHG".to_vec();
    out.extend_from_slice(&[1, 0, kind, 0]);
    match kind {
        1 => [GRANT_ID, [0; 16]].iter().for_each(|i| out.extend_from_slice(i)),
        _ => [[0x22; 16], GRANT_ID].iter().for_each(|i| out.extend_from_slice(i)),
    }
    out.extend_from_slice(&FLEET);
    out.extend_from_slice(&subject.digest());
    put8(&mut out, b"ek");
    put8(&mut out, b"operator");
    out.push(CAPS.len() as u8);
    CAPS.iter().for_each(|c| put8(&mut out, c));
    out.extend_from_slice(&1_000u64.to_le_bytes());
    out.extend_from_slice(&(1u64 << 40).to_le_bytes());
    out.extend_from_slice(&keys::role().id());
    let sig = keys::role().sign(&labelled(Label::Grant, &out));
    out.extend_from_slice(&sig);
    out.push(0);
    out
}

pub fn base() -> Vec<u8> {
    grant(1, keys::user())
}

pub fn escalation() -> Vec<u8> {
    grant(2, keys::user())
}
