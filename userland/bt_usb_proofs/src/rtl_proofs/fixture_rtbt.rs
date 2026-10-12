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

//! Second-layout patch files made to the layout, an RTL8852BU's ROM to
//! take them, and a one-chip table pinning them.

use super::fixture::{pin, Mem};
use super::rom_sim::RomSim;
use super::rtl::chip::{Chip, Format};

/// A subsection: eco, priority, key id and data.
pub type Sub<'a> = (u8, u8, u8, &'a [u8]);
pub const FW: &str = "rtl8852bu_fw.bin";

/// A second-layout file, its extension block naming `project`.
pub fn rtbt(sections: &[(u32, &[Sub<'_>])], project: u8) -> Vec<u8> {
    let mut f = b"RTBTCore".to_vec();
    f.extend([0xd6, 0x25, 0x35, 0x01, 0xf8, 0xc3, 0x02, 0x00]);
    f.extend((sections.len() as u32).to_le_bytes());
    for (opcode, subs) in sections {
        let mut body = (subs.len() as u16).to_le_bytes().to_vec();
        body.extend([0, 0]);
        for (eco, prio, key, data) in subs.iter() {
            body.extend([*eco, *prio, *key, 0]);
            body.extend((data.len() as u32).to_le_bytes());
            body.extend_from_slice(data);
        }
        f.extend(opcode.to_le_bytes());
        f.extend((body.len() as u32).to_le_bytes());
        f.extend(body);
    }
    f.extend([0xff, project, 0x01, 0x00, 0x51, 0x04, 0xfd, 0x77]);
    f
}

/// An RTL8852BU's ROM, version 1, with no key.
pub fn rom() -> RomSim {
    let mut sim = RomSim::rtl8821c();
    (sim.lmp_subversion, sim.hci_revision, sim.hci_version) = (0x8852, 0x000b, 0x0b);
    sim.patched = Some(0x6a7f);
    sim
}

/// A table of one RTL8852BU pinning `file`, and a store holding it.
pub fn ours(file: &[u8]) -> (&'static [Chip], Mem) {
    let chip = Chip {
        name: "RTL8852BU",
        lmp_subversion: 0x8852,
        hci_revision: 0x000b,
        hci_version: 0x0b,
        project: 20,
        format: Format::RtbtCore,
        firmware: pin(FW, file),
        config: None,
        config_needed: false,
    };
    let mut m = Mem::default();
    m.files.insert(FW, file.to_vec());
    (Box::leak(vec![chip].into_boxed_slice()), m)
}
