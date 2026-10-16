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

//! An input report in report mode: the keys its layout says are held,
//! against those it last said, then its pointer's motion, wheel and
//! buttons. A report the layout cannot read changes nothing, as a phantom
//! boot report does not.

use super::state::Host;
use crate::report::{map, Input};
use alloc::vec::Vec;

impl Host {
    pub(super) fn report_in(&mut self, handle: u16, value: &[u8]) -> Vec<Input> {
        let Some(r) = self.inputs.iter_mut().find(|r| r.value == handle) else { return Vec::new() };
        let Some(layout) = self.layouts.get(r.layout) else { return Vec::new() };
        let mut out = Vec::new();
        if let Some(keys) = map::held(layout, r.id, value).filter(|_| r.keys) {
            out = r.held.to(keys);
        }
        if let Some(p) = map::pointer(layout, r.id, value).filter(|_| r.points) {
            out.extend(r.mouse.moved(p.buttons, p.dx, p.dy, p.wheel));
        }
        out
    }
}
