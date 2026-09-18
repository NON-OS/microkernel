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

//! What a disk holds before it is written, read from its first sectors: a
//! GPT with a NONOS partition, some other GPT, an MBR, or nothing a
//! partition tool would recognise. Said beside the disk so a person
//! erasing it knows what they are erasing.

use nonos_disk::PARTITION_NAME;

use crate::device::BlockDevice;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Contents {
    Blank,
    Nonos,
    OtherGpt,
    Mbr,
    Unknown,
}

impl Contents {
    pub fn probe(device: &BlockDevice) -> Contents {
        let mut head = [0u8; 3 * 512];
        if device.sectors < 34 || device.read(0, &mut head).is_err() {
            return Contents::Unknown;
        }
        let (mbr, gpt, entry) = (&head[..512], &head[512..1024], &head[1024..1152]);
        if &gpt[0..8] == b"EFI PART" {
            let mut name = [0u8; 36];
            for (i, ch) in entry[56..128].chunks(2).enumerate() {
                name[i] = if ch[1] == 0 { ch[0] } else { b'?' };
            }
            let len = name.iter().position(|&b| b == 0).unwrap_or(36);
            return if &name[..len] == PARTITION_NAME.as_bytes() {
                Contents::Nonos
            } else {
                Contents::OtherGpt
            };
        }
        if mbr[510] == 0x55 && mbr[511] == 0xAA && mbr[446..510].iter().any(|&b| b != 0) {
            return Contents::Mbr;
        }
        if head.iter().all(|&b| b == 0) {
            return Contents::Blank;
        }
        Contents::Unknown
    }

    pub fn text(self) -> &'static str {
        match self {
            Contents::Blank => "blank",
            Contents::Nonos => "NONOS installed",
            Contents::OtherGpt => "another system (GPT)",
            Contents::Mbr => "another system (MBR)",
            Contents::Unknown => "unrecognised contents",
        }
    }
}
