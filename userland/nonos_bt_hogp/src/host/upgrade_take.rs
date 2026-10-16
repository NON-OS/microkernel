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

//! Whether a service goes to report mode, once its map and Reports are
//! read. It does only if the map parses, and some input Report that can
//! notify, has its CCCD, and holds Keyboard page keys by the map. Then
//! those input Reports, and any holding Consumer keys, take the boot
//! keyboard's place; the output Reports the map gives LEDs take the boot
//! output's; and its Protocol Mode is written to report mode. Anything
//! less keeps boot mode, which needs no map.

use super::hid::{Rep, ReportIn};
use super::state::Host;
use super::step::Step;
use super::types::{Kind, Output, Source};
use crate::error::HogpError;
use crate::report::map::{self, has_keys, types, Layout};
use crate::report::Held;
use crate::uuids::{prop, REPORT_PROTOCOL};
use alloc::vec::Vec;

impl Host {
    pub(super) fn take_up(&mut self, service: usize, now: u64) -> Result<Step, HogpError> {
        if let Ok(layout) = map::parse(&core::mem::take(&mut self.map)) {
            self.switch(service, layout);
        }
        self.upgrade(service + 1, now)
    }

    fn switch(&mut self, service: usize, layout: Layout) {
        let hid = &self.hids[service];
        let input = |r: &&Rep| match r.reference {
            Some((id, 1)) => {
                r.props & prop::NOTIFY != 0 && r.cccd.is_some() && has_keys(&layout, id)
            }
            _ => false,
        };
        let inputs: Vec<Rep> = hid.reports.iter().filter(input).copied().collect();
        if !inputs.iter().any(|r| r.reference.is_some_and(|(id, _)| types(&layout, id))) {
            return;
        }
        let at = self.layouts.len();
        self.sources.retain(|s| Some(s.value) != hid.keyboard);
        for r in &inputs {
            let (value, id) = (r.value, r.reference.map_or(0, |x| x.0));
            self.sources.push(Source { kind: Kind::Report, value, end: r.end, cccd: r.cccd });
            self.inputs.push(ReportIn { value, layout: at, id, held: Held::default() });
        }
        for m in self.modes.iter_mut().filter(|m| Some(m.0) == hid.mode) {
            m.1 = REPORT_PROTOCOL;
        }
        self.outputs.retain(|o| !hid.outputs.contains(&o.handle));
        for r in hid.reports.iter().filter(|r| r.props & prop::WRITE_WITHOUT_RESPONSE != 0) {
            if let Some((id, 2)) = r.reference.filter(|x| map::leds(&layout, x.0, 0).is_some()) {
                self.outputs.push(Output { handle: r.value, report: Some((at, id)) });
            }
        }
        self.layouts.push(layout);
    }
}
