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

//! When a service's declarations are all read: its boot input reports and
//! Protocol Mode and boot keyboard output reports are kept, each report ending where the next declaration
//! starts (Vol 3, Part G, 3.3). Then the next service, or the CCCDs.

use super::state::Host;
use super::step::Step;
use super::types::{Kind, Source};
use crate::error::HogpError;
use crate::uuids::{
    prop, BOOT_KEYBOARD_INPUT, BOOT_KEYBOARD_OUTPUT, BOOT_MOUSE_INPUT, PROTOCOL_MODE,
};

impl Host {
    pub(super) fn service_done(&mut self, service: usize, now_ms: u64) -> Result<Step, HogpError> {
        let end = self.services.get(service).ok_or(HogpError::Protocol)?.1;
        let decls = core::mem::take(&mut self.decls);
        for (i, d) in decls.iter().enumerate() {
            let last = decls.get(i + 1).map_or(end, |n| n.handle - 1);
            let kind = match d.uuid {
                Some(BOOT_KEYBOARD_INPUT) => Kind::Keyboard,
                Some(BOOT_MOUSE_INPUT) => Kind::Mouse,
                Some(PROTOCOL_MODE) if d.props & prop::WRITE_WITHOUT_RESPONSE != 0 => {
                    self.modes.push(d.value);
                    continue;
                }
                Some(BOOT_KEYBOARD_OUTPUT) if d.props & prop::WRITE_WITHOUT_RESPONSE != 0 => {
                    self.outputs.push(d.value);
                    continue;
                }
                _ => continue,
            };
            // A report that cannot notify is of no use to a host that
            // never polls.
            if d.props & prop::NOTIFY != 0 {
                self.sources.push(Source { kind, value: d.value, end: last, cccd: None });
            }
        }
        match self.services.get(service + 1) {
            Some(&(start, _)) => Ok(self.read_chars(service + 1, start, now_ms)),
            None if self.sources.is_empty() => Err(HogpError::NoHid),
            None => self.find_cccd(0, now_ms),
        }
    }
}
