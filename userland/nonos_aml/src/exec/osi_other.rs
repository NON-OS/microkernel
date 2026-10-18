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

//! The feature group strings section 5.7.2 defines, and other operating
//! systems' names. A feature string is answered only once NONOS does what
//! it promises; until the owner rules, none is (see the F-99 letter's open
//! questions). The firmware behaviour in each reason is recalled.

use super::{Group, Honour, Row};

const fn row(
    string: &'static [u8],
    group: Group,
    honour: Honour,
    unlocks: &'static str,
    why: &'static str,
) -> Row {
    Row { string, group, answered: false, honour, unlocks, verified: false, why }
}

const FOREIGN: &str = "paths tested only against another system's drivers";

pub const ROWS: &[Row] = &[
    row(
        b"Module Device",
        Group::Feature,
        Honour::No,
        "ACPI0004 modules",
        "NONOS gives them no meaning",
    ),
    row(b"Processor Device", Group::Feature, Honour::Yes, "", "ACPI0007: cpu/find reads them"),
    row(b"3.0 Thermal Model", Group::Feature, Honour::No, "_TZD and _TZP", "cooling reads neither"),
    row(
        b"3.0 _SCP Extensions",
        Group::Feature,
        Honour::No,
        "_SCP limits",
        "NONOS sends one argument",
    ),
    row(
        b"Processor Aggregator Device",
        Group::Feature,
        Honour::No,
        "ACPI000C core idling on request",
        "nothing answers its requests",
    ),
    row(
        b"Extended Address Space Descriptor",
        Group::Feature,
        Honour::Yes,
        "",
        "resource 0x8B: motherboard/space reads it; that 5.7.2 lists it is recalled only",
    ),
    row(b"Linux", Group::Vendor, Honour::No, FOREIGN, "another system's name"),
    row(b"Darwin", Group::Vendor, Honour::No, FOREIGN, "another system's name"),
];
