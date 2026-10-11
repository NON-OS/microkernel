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

//! Notify (section 19.6.95): queued for the capsule, never acted on here.

use crate::exec::{Fault, Host, Machine};
use crate::load::Aml;
use alloc::collections::VecDeque;

impl<H: Host> Machine<'_, H> {
    /// The object is a Device, Processor or ThermalZone named directly or
    /// passed in a local or argument. A full queue drops the new notification
    /// and counts it, so the capsule sees that it fell behind.
    pub(crate) fn notify(&mut self, aml: &mut Aml, t: u16, node: u32) -> Result<(), Fault> {
        let _at = aml.node(t, node).at;
        let (a, b) = (aml.kid(t, node, 0), aml.kid(t, node, 1));
        let id = self.object_of(aml, t, a, None)?;
        let value = self.int(aml, t, b)? as u8;
        let q: &mut VecDeque<_> = &mut aml.rt.notifications;
        if q.len() < crate::exec::Runtime::MAX_NOTIFICATIONS {
            q.push_back((id, value));
        } else {
            aml.rt.dropped_notifications = aml.rt.dropped_notifications.saturating_add(1);
        }
        Ok(())
    }
}
