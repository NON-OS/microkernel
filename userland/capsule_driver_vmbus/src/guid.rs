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

//! Interface GUIDs, as VMBus carries them (the first three fields little
//! endian), and the devices they name.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Guid(pub [u8; 16]);

impl Guid {
    pub const fn new(a: u32, b: u16, c: u16, d: [u8; 8]) -> Self {
        let a = a.to_le_bytes();
        let b = b.to_le_bytes();
        let c = c.to_le_bytes();
        Self([
            a[0], a[1], a[2], a[3], b[0], b[1], c[0], c[1], d[0], d[1], d[2], d[3], d[4], d[5],
            d[6], d[7],
        ])
    }

    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        Some(Self(b.get(..16)?.try_into().ok()?))
    }
}

/// The interfaces a Hyper-V host offers, by the name the console gives.
const KNOWN: [(Guid, &str); 14] = [
    (
        Guid::new(0xf861_5163, 0xdf3e, 0x46c5, [0x91, 0x3f, 0xf2, 0xd2, 0xf9, 0x65, 0xed, 0x0e]),
        "network",
    ),
    (
        Guid::new(0xba61_63d9, 0x04a1, 0x4d29, [0xb6, 0x05, 0x72, 0xe2, 0xff, 0xb1, 0xdc, 0x7f]),
        "scsi",
    ),
    (
        Guid::new(0x3241_2632, 0x86cb, 0x44a2, [0x9b, 0x5c, 0x50, 0xd1, 0x41, 0x73, 0x54, 0xf5]),
        "ide",
    ),
    (
        Guid::new(0xda0a_7802, 0xe377, 0x4aac, [0x8e, 0x77, 0x05, 0x58, 0xeb, 0x10, 0x73, 0xf8]),
        "video",
    ),
    (
        Guid::new(0xf912_ad6d, 0x2b17, 0x48ea, [0xbd, 0x65, 0xf9, 0x27, 0xa6, 0x1c, 0x76, 0x84]),
        "keyboard",
    ),
    (
        Guid::new(0xcfa8_b69e, 0x5b4a, 0x4cc0, [0xb9, 0x8b, 0x8b, 0xa1, 0xa1, 0xf3, 0xf9, 0x5a]),
        "mouse",
    ),
    (
        Guid::new(0x5716_4f39, 0x9115, 0x4e78, [0xab, 0x55, 0x38, 0x2f, 0x3b, 0xd5, 0x42, 0x2d]),
        "heartbeat",
    ),
    (
        Guid::new(0x0e0b_6031, 0x5213, 0x4934, [0x81, 0x8b, 0x38, 0xd9, 0x0c, 0xed, 0x39, 0xdb]),
        "shutdown",
    ),
    (
        Guid::new(0x9527_e630, 0xd0ae, 0x497b, [0xad, 0xce, 0xe8, 0x0a, 0xb0, 0x17, 0x5c, 0xaf]),
        "time sync",
    ),
    (
        Guid::new(0xa9a0_f4e7, 0x5a45, 0x4d96, [0xb8, 0x27, 0x8a, 0x84, 0x1e, 0x8c, 0x03, 0xe6]),
        "key-value exchange",
    ),
    (
        Guid::new(0x35fa_2e29, 0xea23, 0x4236, [0x96, 0xae, 0x3a, 0x6e, 0xba, 0xcb, 0xa4, 0x40]),
        "backup (VSS)",
    ),
    (
        Guid::new(0x5250_74dc, 0x8985, 0x46e2, [0x80, 0x57, 0xa3, 0x07, 0xdc, 0x18, 0xa5, 0x02]),
        "dynamic memory",
    ),
    (
        Guid::new(0x34d1_4be3, 0xdee4, 0x41c8, [0x9a, 0xe7, 0x6b, 0x17, 0x49, 0x77, 0xc1, 0x92]),
        "file copy",
    ),
    (
        Guid::new(0x44c4_f61d, 0x4444, 0x4400, [0x9d, 0x52, 0x80, 0x2e, 0x27, 0xed, 0xe1, 0x9f]),
        "pci",
    ),
];

pub fn name(g: Guid) -> &'static str {
    KNOWN.iter().find(|(k, _)| *k == g).map_or("unknown", |(_, n)| n)
}
