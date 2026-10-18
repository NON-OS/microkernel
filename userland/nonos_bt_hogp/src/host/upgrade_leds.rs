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

//! An upgraded service's lock LEDs: its boot keyboard output reports are
//! not read in report mode, so the output Reports whose map fields carry
//! LED usages, and that take a Write Command, are written instead.

use super::state::Host;
use super::types::Output;
use crate::report::map::{self, Layout};
use crate::uuids::prop;

impl Host {
    pub(super) fn take_leds(&mut self, service: usize, at: usize, layout: &Layout) {
        let hid = &self.hids[service];
        self.outputs.retain(|o| !hid.outputs.contains(&o.handle));
        for r in hid.reports.iter().filter(|r| r.props & prop::WRITE_WITHOUT_RESPONSE != 0) {
            if let Some((id, 2)) = r.reference.filter(|x| map::leds(layout, x.0, 0).is_some()) {
                self.outputs.push(Output { handle: r.value, report: Some((at, id)) });
            }
        }
    }
}
