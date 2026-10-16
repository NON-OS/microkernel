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

//! When a service's declarations are all read: its boot input reports,
//! Protocol Mode and boot keyboard output reports are kept, and for report
//! mode its Report Map and Reports, each characteristic ending where the
//! next declaration starts (Vol 3, Part G, 3.3). Then the next service,
//! or report mode.

use super::hid::{Hid, Rep, REPORTS_MAX};
use super::state::Host;
use super::step::Step;
use super::types::{Kind, Output, Source};
use crate::error::HogpError;
use crate::uuids::{prop::*, *};

impl Host {
    pub(super) fn service_done(&mut self, service: usize, now_ms: u64) -> Result<Step, HogpError> {
        let end = self.services.get(service).ok_or(HogpError::Protocol)?.1;
        let decls = core::mem::take(&mut self.decls);
        let mut hid = Hid::default();
        for (i, d) in decls.iter().enumerate() {
            let last = decls.get(i + 1).map_or(end, |n| n.handle - 1);
            let (has, value) = (|p: u8| d.props & p != 0, d.value);
            let source = |kind| Source { kind, value, end: last, cccd: None };
            match d.uuid {
                // A report that cannot notify is of no use to a host that
                // never polls.
                Some(BOOT_KEYBOARD_INPUT) if has(NOTIFY) => {
                    self.sources.push(source(Kind::Keyboard));
                    hid.keyboard = Some(value);
                }
                Some(BOOT_MOUSE_INPUT) if has(NOTIFY) => {
                    self.sources.push(source(Kind::Mouse));
                    hid.mouse = Some(value);
                }
                Some(PROTOCOL_MODE) if has(WRITE_WITHOUT_RESPONSE) => {
                    self.modes.push((value, BOOT_PROTOCOL));
                    hid.mode = Some(value);
                }
                Some(BOOT_KEYBOARD_OUTPUT) if has(WRITE_WITHOUT_RESPONSE) => {
                    self.outputs.push(Output { handle: value, report: None });
                    hid.outputs.push(value);
                }
                Some(REPORT_MAP) if has(READ) && hid.map.is_none() => hid.map = Some(value),
                Some(REPORT) if hid.reports.len() < REPORTS_MAX => {
                    hid.reports.push(Rep { value, end: last, props: d.props, ..Rep::default() })
                }
                _ => {}
            }
        }
        self.hids.push(hid);
        match self.services.get(service + 1) {
            Some(&(start, _)) => Ok(self.read_chars(service + 1, start, now_ms)),
            None if self.sources.is_empty() => Err(HogpError::NoHid),
            None => self.upgrade(0, now_ms),
        }
    }
}
