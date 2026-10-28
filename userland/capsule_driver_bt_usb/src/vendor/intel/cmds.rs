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

//! The commands after an .sfi file's headers: HCI command packets, an
//! opcode, a length octet and that many parameters, to the end of the file.
//! The file holds Intel NOPs so they fall into runs of whole commands whose
//! lengths are multiples of four, which Secure Send needs; Intel Write Boot
//! Params among them names where the firmware boots.

use alloc::vec::Vec;

const WRITE_BOOT_PARAMS: u16 = 0xfc0e;

/// The commands cut into runs whose length is a multiple of four, each
/// whole commands and sent as one Data section; `None` if a command runs
/// past the end or the last run is not aligned.
pub fn runs(cmds: &[u8]) -> Option<Vec<&[u8]>> {
    let (mut out, mut at, mut len) = (Vec::new(), 0, 0);
    while at + len < cmds.len() {
        len += 3 + usize::from(*cmds.get(at + len + 2)?);
        let run = cmds.get(at..at + len)?;
        if len % 4 == 0 {
            out.push(run);
            (at, len) = (at + len, 0);
        }
    }
    (len == 0).then_some(out)
}

/// Intel Write Boot Params (0xfc0e) among the commands: the address the
/// firmware boots from, and its build number, week and year
/// (`btintel_firmware_version`).
pub fn boot_params(mut cmds: &[u8]) -> Option<(u32, [u8; 3])> {
    while let [lo, hi, len, rest @ ..] = cmds {
        let p = rest.get(..usize::from(*len))?;
        if let (WRITE_BOOT_PARAMS, &[a, b, c, d, n, w, y, ..]) = (u16::from_le_bytes([*lo, *hi]), p)
        {
            return Some((u32::from_le_bytes([a, b, c, d]), [n, w, y]));
        }
        cmds = &rest[p.len()..];
    }
    None
}
