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

//! Which legacy devices NONOS publishes, by the PnP ids firmware names
//! them with (_HID, else a _CID): the 8042's keyboard and mouse ports, the
//! 16550 UART, the CMOS RTC, the HPET and the 8254 PIT.

use crate::platform::HardwareId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegacyKind {
    Keyboard,
    Mouse,
    Uart,
    Rtc,
    Hpet,
    Pit,
}

const IDS: &[(&[u8], LegacyKind)] = &[
    (b"PNP0303", LegacyKind::Keyboard),
    (b"PNP0F13", LegacyKind::Mouse),
    (b"PNP0F03", LegacyKind::Mouse),
    (b"PNP0501", LegacyKind::Uart),
    (b"PNP0500", LegacyKind::Uart),
    (b"PNP0B00", LegacyKind::Rtc),
    (b"PNP0103", LegacyKind::Hpet),
    (b"PNP0100", LegacyKind::Pit),
];

/// The first id that names a legacy device decides; a vendor's own _HID
/// with PNP0303 as its _CID is a keyboard all the same.
pub fn kind_of(ids: &[HardwareId]) -> Option<LegacyKind> {
    ids.iter().find_map(|h| IDS.iter().find(|(id, _)| *id == h.bytes()).map(|(_, k)| *k))
}
