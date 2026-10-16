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

//! What one HID service offers for report mode (HIDS 1.0, 3), kept while
//! its declarations are read, and the report state once it is upgraded.

use crate::report::Held;
use crate::uuids::prop;
use alloc::vec::Vec;

/// Report characteristics kept for one service; more are passed over.
pub const REPORTS_MAX: usize = 16;

/// One Report characteristic: its value handle, its last handle, its
/// properties, and what its descriptors say once read.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Rep {
    pub value: u16,
    pub end: u16,
    pub props: u8,
    pub refh: Option<u16>,
    pub cccd: Option<u16>,
    /// The Report Reference: Report ID and type, 1 input, 2 output.
    pub reference: Option<(u8, u8)>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Hid {
    pub mode: Option<u16>,
    pub map: Option<u16>,
    /// The boot keyboard input report's value handle.
    pub keyboard: Option<u16>,
    pub mouse: bool,
    /// Boot keyboard output reports, which report mode leaves unread.
    pub outputs: Vec<u16>,
    pub reports: Vec<Rep>,
}

impl Hid {
    /// Only a keyboard is upgraded: Protocol Mode is one per service, and
    /// report mode would silence a boot mouse there that has no map
    /// decoding. A service with no mode, map or notifying report keeps
    /// boot mode.
    pub fn worth(&self) -> bool {
        let notifies = self.reports.iter().any(|r| r.props & prop::NOTIFY != 0);
        self.mode.is_some()
            && self.map.is_some()
            && self.keyboard.is_some()
            && !self.mouse
            && notifies
    }
}

/// An input report in report mode: its value handle, its service's layout
/// and its Report ID, and the keys it last said were held.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ReportIn {
    pub value: u16,
    pub layout: usize,
    pub id: u8,
    pub held: Held,
}
