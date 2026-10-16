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
//! read. The input Reports that count notify, have their CCCD, and hold
//! keys or a relative pointer by the map. One must count, and they must
//! replace every boot report the service has: a boot keyboard needs one
//! with Keyboard page keys, a boot mouse one with relative X and Y. Then
//! they take the boot reports' place, and any Protocol Mode is written.

use super::hid::Rep;
use super::report_in::ReportIn;
use super::state::Host;
use super::types::{Kind, Source};
use crate::report::map::{has_keys, points, types, Layout};
use crate::report::{Held, Mouse};
use crate::uuids::{prop, REPORT_PROTOCOL};
use alloc::vec::Vec;

impl Host {
    pub(super) fn switch(&mut self, service: usize, layout: Layout) {
        let hid = &self.hids[service];
        let input = |r: &&Rep| match r.reference {
            Some((id, 1)) => {
                let used = has_keys(&layout, id) || points(&layout, id);
                r.props & prop::NOTIFY != 0 && r.cccd.is_some() && used
            }
            _ => false,
        };
        let inputs: Vec<Rep> = hid.reports.iter().filter(input).copied().collect();
        let any = |f: fn(&Layout, u8) -> bool| {
            inputs.iter().any(|r| r.reference.is_some_and(|(id, _)| f(&layout, id)))
        };
        let lost = |boot: Option<u16>, f| boot.is_some() && !any(f);
        if inputs.is_empty() || lost(hid.keyboard, types) || lost(hid.mouse, points) {
            return;
        }
        let at = self.layouts.len();
        let boot = [hid.keyboard, hid.mouse];
        self.sources.retain(|s| !boot.contains(&Some(s.value)));
        for r in &inputs {
            let (value, id) = (r.value, r.reference.map_or(0, |x| x.0));
            let (keys, points) = (has_keys(&layout, id), points(&layout, id));
            self.sources.push(Source { kind: Kind::Report, value, end: r.end, cccd: r.cccd });
            let (held, mouse) = (Held::default(), Mouse::default());
            self.inputs.push(ReportIn { value, layout: at, id, keys, points, held, mouse });
        }
        for m in self.modes.iter_mut().filter(|m| Some(m.0) == hid.mode) {
            m.1 = REPORT_PROTOCOL;
        }
        self.take_leds(service, at, &layout);
        self.layouts.push(layout);
    }
}
