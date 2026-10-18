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
//! A desk for the Bluetooth bond rule: events stamped the way the
//! Bluetooth capsule stamps them, and every frame sent read back with its
//! envelope flags, so the router's notice to the shell can be told apart.

use crate::route::bond::{BOND_AUTHENTICATED, SOURCE_BT};
use crate::route::chord::KEY_DELETE;
use crate::router_desk::{key_down, key_up, MOD_ALT, MOD_CTRL};
use crate::state::Context;

/// A Bluetooth keyboard paired by Just Works; one paired authenticated.
pub const JUST_WORKS: u16 = SOURCE_BT;
pub const PASSKEY: u16 = SOURCE_BT | BOND_AUTHENTICATED;
pub const CHORD: u16 = MOD_CTRL | MOD_ALT;

/// One frame as sent: to whom, its envelope flags, and its event.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Out {
    pub pid: u32,
    pub env: u16,
    pub kind: u16,
    pub flags: u16,
    pub code: u32,
}

/// Everything sent since the last call, in order.
pub fn out() -> Vec<Out> {
    let u16_at = |f: &[u8], at: usize| u16::from_le_bytes([f[at], f[at + 1]]);
    nonos_libc::take_sent()
        .into_iter()
        .map(|(pid, f)| Out {
            pid,
            env: u16_at(&f, 6),
            kind: u16_at(&f, 8),
            flags: u16_at(&f, 10),
            code: u32::from_le_bytes([f[12], f[13], f[14], f[15]]),
        })
        .collect()
}

/// The router's notices among them, by receiver.
pub fn notices(sent: &[Out]) -> Vec<u32> {
    sent.iter().filter(|o| o.env != 0).map(|o| o.pid).collect()
}

/// The events among them (no notice), as (pid, code).
pub fn events(sent: &[Out]) -> Vec<(u32, u32)> {
    sent.iter().filter(|o| o.env == 0).map(|o| (o.pid, o.code)).collect()
}

/// Ctrl+Alt+Delete pressed and let go on a keyboard stamped `bond`.
pub fn chord(ctx: &mut Context, bond: u16) {
    key_down(ctx, KEY_DELETE, CHORD | bond);
    key_up(ctx, KEY_DELETE, CHORD | bond);
}
