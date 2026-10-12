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

//! A Realtek controller this capsule patches, known by what its ROM answers
//! to Read Local Version Information (Vol 4, Part E, 7.4.1). Once patched
//! the controller answers with another LMP subversion, so a controller that
//! kept its patch from before matches nothing and is left as it is.

use super::pin::Pin;
use super::version::Version;

/// Realtek Semiconductor Corporation in the Bluetooth Assigned Numbers'
/// company identifiers.
pub const REALTEK: u16 = 0x005d;

#[derive(Debug)]
pub struct Chip {
    pub name: &'static str,
    pub lmp_subversion: u16,
    pub hci_revision: u16,
    pub hci_version: u8,
    /// The project the patch file names in its extension block; a file for
    /// another project is not sent, whatever its name.
    pub project: u8,
    pub format: Format,
    pub firmware: &'static Pin,
    pub config: Option<&'static Pin>,
    /// Some chips do not start without their config; others take it when
    /// the store has it and start without it when it does not.
    pub config_needed: bool,
}

/// Which of Realtek's two patch file layouts the chip's file is in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    /// "Realtech": a patch per ROM version, chosen by a table.
    Realtech,
    /// "RTBTCore": sections of subsections, some signed for a key id.
    RtbtCore,
}

/// The chip whose ROM answered `v`; none for another company's controller,
/// a Realtek chip not in `chips`, or one already patched.
pub fn rom<'a>(chips: &'a [Chip], v: &Version) -> Option<&'a Chip> {
    if v.company != REALTEK {
        return None;
    }
    chips.iter().find(|c| {
        (c.lmp_subversion, c.hci_revision, c.hci_version)
            == (v.lmp_subversion, v.hci_revision, v.hci_version)
    })
}
