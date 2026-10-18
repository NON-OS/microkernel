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

//! Microsoft's release strings, oldest first, as published in "How to
//! Identify the Windows Version in ACPI by Using _OSI" and recalled from
//! it (the page could not be read here; from "Windows 2018" on even the
//! strings are unverified). Windows answers its own string and every
//! earlier one, so firmware reads a yes as "this release or later", and a
//! yes to any string opens what every earlier one opens.

use super::{Group, Honour, Row};

/// Answered; honoured wholly when it opens nothing NONOS lacks.
const fn win(string: &'static [u8], unlocks: &'static str, why: &'static str) -> Row {
    let honour = if unlocks.is_empty() { Honour::Yes } else { Honour::Partly };
    Row { string, group: Group::Windows, answered: true, honour, unlocks, verified: false, why }
}

const S0: &str =
    "S0 low-power idle entry (the LPS0 _DSM calls); NONOS reads LPIT but never enters it";
const TEN: &str = "S0 low-power idle entry; USB4 and Thunderbolt connection made by software";
const HOTPLUG: &str = "native PCIe hot-plug once _OSC grants it (built, off by default)";
const LADDER: &str = "10: no new path known; answered so the year ladder lands on the newest";

pub const ROWS: &[Row] = &[
    win(b"Windows 2000", "", "the base paths: EC, battery, lid, thermal, buttons"),
    win(b"Windows 2001", "", "XP: as 2000; some firmware gates EC brightness keys here"),
    win(b"Windows 2001 SP1", "", "XP SP1: no further path known"),
    win(b"Windows 2001.1", "", "Server 2003: no further path known"),
    win(b"Windows 2001 SP2", "", "XP SP2: no further path known"),
    win(b"Windows 2001.1 SP1", "", "Server 2003 SP1: no further path known"),
    win(b"Windows 2006", "", "Vista: backlight by _BCL, _BCM and _DOS, which NONOS drives"),
    win(b"Windows 2006.1", "", "Server 2008: as Vista"),
    win(b"Windows 2006 SP1", "", "Vista SP1: as Vista"),
    win(b"Windows 2006 SP2", "", "Vista SP2: as Vista"),
    win(b"Windows 2009", HOTPLUG, "7: some firmware grants native PCIe control from here"),
    win(b"Windows 2012", S0, "8: GPIO-signalled events, HID over I2C, GPIO buttons"),
    win(b"Windows 2013", S0, "8.1: as 8, S0 low-power idle more often the only sleep"),
    win(b"Windows 2015", TEN, "10: UCSI for USB-C and the Intel HID buttons, which NONOS runs"),
    win(b"Windows 2016", TEN, LADDER),
    win(b"Windows 2017", TEN, LADDER),
    win(b"Windows 2017.2", TEN, LADDER),
    win(b"Windows 2018", TEN, LADDER),
    win(b"Windows 2018.2", TEN, LADDER),
    win(b"Windows 2019", TEN, LADDER),
    win(b"Windows 2020", TEN, LADDER),
    win(b"Windows 2021", TEN, "11: some firmware gates its USB4 paths on the Windows 11 strings"),
    win(b"Windows 2022", TEN, "11 22H2: as 2021"),
];
