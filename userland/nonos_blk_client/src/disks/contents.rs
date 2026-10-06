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
//! erasing it knows what they are erasing. A read that fails is said as
//! such and the disk is not offered: on the HP (6 Oct) a drive that had
//! stopped answering showed as "unrecognised contents", was offered, and
//! stalled the install at 2% with status -110. Pure, for the host proofs.

use nonos_disk::written_by_nonos;

/// The MBR, the GPT header and the first sector of entries.
pub const HEAD: usize = 3 * 512;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Contents {
    Blank,
    Nonos,
    OtherGpt,
    Mbr,
    Unknown,
    /// The read of its first sectors failed with this driver status.
    Unread(i32),
}

impl Contents {
    /// A disk of `sectors`, its first sectors read by `read`, which answers
    /// the driver's status when it fails.
    pub fn read_with(sectors: u64, read: impl FnOnce(&mut [u8; HEAD]) -> Result<(), i32>) -> Self {
        let mut head = [0u8; HEAD];
        if sectors < 34 {
            return Contents::Unknown;
        }
        match read(&mut head) {
            Ok(()) => Contents::of(&head),
            Err(status) => Contents::Unread(status),
        }
    }

    /// What the MBR, the GPT header and the first entry say.
    pub fn of(head: &[u8; HEAD]) -> Contents {
        let (mbr, gpt, entry) = (&head[..512], &head[512..1024], &head[1024..1152]);
        if &gpt[0..8] == b"EFI PART" {
            return if written_by_nonos(entry) { Contents::Nonos } else { Contents::OtherGpt };
        }
        if mbr[510] == 0x55 && mbr[511] == 0xAA && mbr[446..510].iter().any(|&b| b != 0) {
            return Contents::Mbr;
        }
        if head.iter().all(|&b| b == 0) {
            return Contents::Blank;
        }
        Contents::Unknown
    }
}
