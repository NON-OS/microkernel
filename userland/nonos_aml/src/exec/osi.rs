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

//! The _OSI strings NONOS answers, and why (section 5.7.2 defines _OSI).
//!
//! Firmware written for Windows asks _OSI for the Windows version strings
//! Microsoft publishes in "How to Identify the Windows Version in ACPI by
//! Using _OSI" (learn.microsoft.com, windows-hardware/drivers/acpi/
//! winacpi-osi), and turns features on or off by the answers; a machine
//! that hears no to all of them often runs in a fallback mode with devices
//! missing. Windows answers true for its own string and every earlier one,
//! so a firmware check `If (_OSI ("Windows 2015"))` means "Windows 10 or
//! later". NONOS answers the same way, up to the newest string in the table.
//!
//! The page could not be read from the build sandbox; the strings below
//! follow the page as the author knows it. Those from "Windows 2018" on are
//! marked unverified in the letter, corroborated only by secondary reports,
//! and are for the owner to check against the page.

/// Microsoft's version strings, oldest first, each with the release it
/// stands for.
pub const WINDOWS: &[&[u8]] = &[
    b"Windows 2000",       // Windows 2000
    b"Windows 2001",       // Windows XP
    b"Windows 2001 SP1",   // Windows XP SP1
    b"Windows 2001.1",     // Windows Server 2003
    b"Windows 2001 SP2",   // Windows XP SP2
    b"Windows 2001.1 SP1", // Windows Server 2003 SP1
    b"Windows 2006",       // Windows Vista
    b"Windows 2006.1",     // Windows Server 2008
    b"Windows 2006 SP1",   // Windows Vista SP1
    b"Windows 2006 SP2",   // Windows Vista SP2
    b"Windows 2009",       // Windows 7, Windows Server 2008 R2
    b"Windows 2012",       // Windows 8, Windows Server 2012
    b"Windows 2013",       // Windows 8.1, Windows Server 2012 R2
    b"Windows 2015",       // Windows 10
    b"Windows 2016",       // Windows 10, version 1607
    b"Windows 2017",       // Windows 10, version 1703
    b"Windows 2017.2",     // Windows 10, version 1709
    b"Windows 2018",       // Windows 10, version 1803
    b"Windows 2018.2",     // Windows 10, version 1809
    b"Windows 2019",       // Windows 10, version 1903
    b"Windows 2020",       // Windows 10, version 2004
    b"Windows 2021",       // Windows 11
    b"Windows 2022",       // Windows 11, version 22H2
];

/// The feature group strings section 5.7.2 defines ("Module Device",
/// "Processor Device", "3.0 Thermal Model", "3.0 _SCP Extensions",
/// "Processor Aggregator Device") are not answered: each promises OSPM
/// support NONOS does not have yet. The unit that brings a feature in adds
/// its string here, so the answer never runs ahead of the code.
pub const FEATURES: &[&[u8]] = &[];

/// No operating system vendor string is answered: "Linux" and its kin
/// select paths firmware tested against another system's drivers, and the
/// specification leaves them to their vendors.
pub const VENDORS: &[&[u8]] = &[];
