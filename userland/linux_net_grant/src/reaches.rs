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

//! Where a stream may go on a grant: anywhere over the anonymity network on
//! a policy's, and only to a box's own destinations on a box's, matched as
//! the personality matches them (nonos_linux_policy::dests::allows).

use nonos_linux_policy::dests::{allows, Target};

use super::Grant;

/// Whether `grant` lets a stream go to `host` (a name, or an IPv4 address in
/// dotted form, as both fronts are given it) on `port`.
pub fn reaches(grant: &Grant, host: &[u8], port: u16) -> bool {
    match grant {
        Grant::Policy { .. } => true,
        Grant::Boxed { dests } => allows(dests, target(host), port),
    }
}

fn target(host: &[u8]) -> Target<'_> {
    dotted(host).map_or(Target::Name(host), Target::V4)
}

fn dotted(host: &[u8]) -> Option<[u8; 4]> {
    let mut out = [0u8; 4];
    let mut parts = host.split(|b| *b == b'.');
    for o in out.iter_mut() {
        let p = parts.next()?;
        if p.is_empty() || p.len() > 3 || !p.iter().all(u8::is_ascii_digit) {
            return None;
        }
        *o = p.iter().try_fold(0u16, |n, d| Some(n * 10 + u16::from(d - b'0')))?.try_into().ok()?;
    }
    parts.next().is_none().then_some(out)
}
