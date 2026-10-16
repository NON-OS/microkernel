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

//! F 12's brightness bytes, served natively (unit 13; the the maintainers' ruling):
//! a request is an op byte and its arguments, a reply an i32 status (0 or
//! a negative errno) then the op's body, little-endian, at most 512 bytes.
//! BRIGHTNESS (3) lists every backlit panel: count u8, then per panel its
//! identity u32, the level now in percent (0xFF unknown), n u8 and n
//! ascending percent levels. SET_BRIGHTNESS (4) takes a percent and maybe
//! a panel; each panel goes to its level nearest it, a tie to the lower.
//! VCP (5) and SET_VCP (6) are a monitor's own controls (`vcp_wire.rs`).
//! An unknown op is EINVAL with no body.

use super::brightness::Brightness;
pub use super::rights::Rights;
use super::vcp_wire::{Controls, OP_SET_VCP, OP_VCP};

pub const OP_BRIGHTNESS: u8 = 3;
pub const OP_SET_BRIGHTNESS: u8 = 4;
pub const EPERM: i32 = -1;
pub const ENOENT: i32 = -2;
pub const EIO: i32 = -5;
pub const EINVAL: i32 = -22;
pub const MAX_REPLY: usize = 512;

/// A panel's identity: this driver cannot name the connector's ACPI
/// _ADR, so its own id, the output, with the top bit set (the ruling).
pub fn identity(output: u16) -> u32 {
    0x8000_0000 | output as u32
}

/// Answer `req` into `out` for a sender with `may`. The bytes written.
pub fn answer<B: Brightness + Controls>(
    b: &mut B,
    outputs: u16,
    req: &[u8],
    may: Rights,
    out: &mut [u8],
) -> usize {
    let (status, body) = match req {
        [OP_BRIGHTNESS] => super::brightness_ops::list(b, outputs, &mut out[4..]),
        [OP_SET_BRIGHTNESS, pct, rest @ ..] if *pct <= 100 => {
            let panel = match rest {
                [] => Some(None),
                [a, b2, c, d] => Some(Some(u32::from_le_bytes([*a, *b2, *c, *d]))),
                _ => None,
            };
            match panel {
                None => (EINVAL, 0),
                Some(_) if !may.set => (EPERM, 0),
                Some(p) => (super::brightness_ops::set(b, outputs, *pct, p), 0),
            }
        }
        [OP_VCP | OP_SET_VCP, ..] => super::vcp_wire::answer(b, outputs, req, may, &mut out[4..]),
        _ => (EINVAL, 0),
    };
    out[..4].copy_from_slice(&status.to_le_bytes());
    4 + body
}
