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

//! The report's lines kept off the screen, and the policy its `[BOX-APP]` line
//! names: the program as its receipt names it (capsule_linux's app_line.rs).
//! The last whole report's is kept, for a headless run to ask nonos.prove for
//! that run's receipt. Like every report line it comes from keyed frames only.

use core::cell::UnsafeCell;

use super::box_hold::BoxHold;

pub const NET_LINE: &[u8] = b"[BOX-NET] ";
pub const APP_LINE: &[u8] = b"[BOX-APP] ";
const POLICY_AT: &[u8] = b"v=1 policy=";
pub const POLICY_LEN: usize = 256;

/// The policy the report's one `[BOX-APP]` line names; None without exactly
/// one such line, or with one that is not 256 bytes of lower-case hex.
pub fn policy(hold: &BoxHold) -> Option<[u8; POLICY_LEN]> {
    let mut lines = hold.held.split(|b| *b == b'\n').filter(|l| l.starts_with(APP_LINE));
    let (Some(line), None) = (lines.next(), lines.next()) else { return None };
    let hex = line.strip_prefix(APP_LINE)?.strip_prefix(POLICY_AT)?;
    let mut out = [0u8; POLICY_LEN];
    (hex.len() == 2 * POLICY_LEN).then_some(())?;
    for (o, pair) in out.iter_mut().zip(hex.chunks(2)) {
        *o = nibble(pair[0])? << 4 | nibble(pair[1])?;
    }
    Some(out)
}

fn nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    }
}

struct Last(UnsafeCell<Option<[u8; POLICY_LEN]>>);

/*
 * Safety: a capsule is single threaded, and the cell is only reached through
 * keep and last, neither of which holds a reference past its return.
 */
unsafe impl Sync for Last {}

static LAST: Last = Last(UnsafeCell::new(None));

/// A whole report drawn: its policy is now the last one, or none.
pub fn keep(hold: &BoxHold) {
    unsafe { *LAST.0.get() = policy(hold) }
}

/// The policy of the last whole report drawn, if it named one.
pub fn last() -> Option<[u8; POLICY_LEN]> {
    unsafe { *LAST.0.get() }
}
