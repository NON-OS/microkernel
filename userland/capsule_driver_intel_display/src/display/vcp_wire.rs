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

//! Two ops beside F 12's brightness bytes, for an external monitor's own
//! controls over DDC/CI, in the same framing (an i32 status, then the
//! body). VCP (5): panel u32 and a control code u8; the body is the value
//! now and the most it takes, u16 each; from power, desktop_shell and
//! Settings alone, since a monitor's values are a small fingerprint.
//! SET_VCP (6): panel u32, code u8, value u16, from power or desktop_shell. Only the controls
//! `ddc::vcp` allows answer; the panel is an output's identity, as BRIGHTNESS
//! lists it.

use nonos_gfx_wire::Errno;

use super::brightness_wire::{identity, Rights, EINVAL, ENOENT, EPERM};

pub const OP_VCP: u8 = 5;
pub const OP_SET_VCP: u8 = 6;

pub trait Controls {
    fn vcp(&mut self, output: u16, code: u8) -> Result<(u16, u16), Errno>;
    fn set_vcp(&mut self, output: u16, code: u8, value: u16) -> Result<(), Errno>;
}

/// The status, and the body written into `out`.
pub fn answer<B: Controls>(
    b: &mut B,
    outputs: u16,
    req: &[u8],
    may: Rights,
    out: &mut [u8],
) -> (i32, usize) {
    let output = |p: &[u8]| {
        let id = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
        (0..outputs).find(|o| identity(*o) == id).ok_or(ENOENT)
    };
    match req {
        [OP_VCP, ..] if !may.read_controls => (EPERM, 0),
        [OP_VCP, p @ .., code] if p.len() == 4 => {
            let got = output(p).and_then(|o| b.vcp(o, *code));
            match (got, out.get_mut(..4)) {
                (Ok((now, max)), Some(body)) => {
                    body[..2].copy_from_slice(&now.to_le_bytes());
                    body[2..].copy_from_slice(&max.to_le_bytes());
                    (0, 4)
                }
                (Err(e), _) => (e, 0),
                (Ok(_), None) => (EINVAL, 0),
            }
        }
        [OP_SET_VCP, p @ .., code, lo, hi] if p.len() == 4 => match output(p) {
            Err(e) => (e, 0),
            Ok(_) if !may.set => (EPERM, 0),
            Ok(o) => (b.set_vcp(o, *code, u16::from_le_bytes([*lo, *hi])).err().unwrap_or(0), 0),
        },
        _ => (EINVAL, 0),
    }
}
