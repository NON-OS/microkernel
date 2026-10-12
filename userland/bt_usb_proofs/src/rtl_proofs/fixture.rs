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

//! Patch files laid out as linux-firmware lays them, pins made for them,
//! and a one-chip table. The real files are never needed here: the
//! layout and the checks are proven on files made to it.

pub use super::mem::Mem;
use super::rtl::chip::{Chip, Format};
use super::rtl::pin::Pin;
use sha2::{Digest, Sha256};

pub const VERSION: u32 = 0x75b8_f098;

/// A first-layout patch file: header, the three tables, the patches one
/// after another, then the extension block naming `project`.
pub fn epatch(version: u32, patches: &[(u16, &[u8])], project: u8) -> Vec<u8> {
    let mut f = b"Realtech".to_vec();
    f.extend(version.to_le_bytes());
    f.extend((patches.len() as u16).to_le_bytes());
    let mut at = 14 + 8 * patches.len();
    patches.iter().for_each(|(id, _)| f.extend(id.to_le_bytes()));
    patches.iter().for_each(|(_, b)| f.extend((b.len() as u16).to_le_bytes()));
    for (_, b) in patches {
        f.extend((at as u32).to_le_bytes());
        at += b.len();
    }
    patches.iter().for_each(|(_, b)| f.extend_from_slice(b));
    f.extend([0xff, project, 0x01, 0x00, 0x51, 0x04, 0xfd, 0x77]);
    f
}

/// A pin for `bytes` under `file`, kept for the rest of the run.
pub fn pin(file: &'static str, bytes: &[u8]) -> &'static Pin {
    let sha256 = Sha256::digest(bytes).into();
    Box::leak(Box::new(Pin { file, len: bytes.len(), sha256 }))
}

/// A table of one chip, an RTL8821C's ROM, with these pins.
pub fn chips(fw: &'static Pin, cfg: Option<&'static Pin>, needed: bool) -> &'static [Chip] {
    let (name, lmp_subversion, hci_revision, hci_version, project) =
        ("RTL8821C", 0x8821, 0xc, 8, 10);
    let (firmware, config, config_needed) = (fw, cfg, needed);
    let chip = Chip {
        name,
        lmp_subversion,
        hci_revision,
        hci_version,
        project,
        format: Format::Realtech,
        firmware,
        config,
        config_needed,
    };
    Box::leak(vec![chip].into_boxed_slice())
}
