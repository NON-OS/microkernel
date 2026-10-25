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

//! _OSI (ACPI 6.5, 5.7.2) as one table: every string NONOS has a view on,
//! whether it is answered, how far NONOS honours what a yes lets firmware
//! do, the firmware paths a yes opens that NONOS does not implement, and
//! why. Firmware behaviour in the reasons is recalled, not read; every row
//! says so. The table is in two parts only to keep each file short.

#[path = "osi_other.rs"]
mod osi_other;
#[path = "osi_windows.rs"]
mod osi_windows;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Group {
    Windows, // Microsoft's release strings: a yes says "this release or later"
    Feature, // the feature group strings section 5.7.2 defines
    Vendor,  // another operating system's name
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Honour {
    Yes,    // everything a yes lets firmware do, NONOS does
    Partly, // some of it; `unlocks` names the rest
    No,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Row {
    pub string: &'static [u8],
    pub group: Group,
    pub answered: bool,
    pub honour: Honour,
    /// The firmware paths a yes opens that NONOS does not implement.
    pub unlocks: &'static str,
    /// Whether the firmware behaviour in `why` was checked against a
    /// source; false for every row today.
    pub verified: bool,
    pub why: &'static str,
}

pub const TABLE: [&[Row]; 2] = [osi_windows::ROWS, osi_other::ROWS];

/// The Windows strings, oldest first, which a Personality answers a prefix of.
pub const WINDOWS_ROWS: usize = osi_windows::ROWS.len();

pub fn rows() -> impl Iterator<Item = &'static Row> {
    TABLE.iter().flat_map(|part| part.iter())
}

pub fn windows() -> impl Iterator<Item = &'static [u8]> {
    osi_windows::ROWS.iter().map(|r| r.string)
}

/// Whether `s` is answered when only the first `windows` Windows strings are.
pub fn answered(s: &[u8], windows: usize) -> bool {
    let win = osi_windows::ROWS.iter().take(windows).any(|r| r.answered && r.string == s);
    win || osi_other::ROWS.iter().any(|r| r.answered && r.string == s)
}
