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
//! A fresh host: no resources, no scanout showing anything, answering every
//! command, its displays scanout 0 at PANEL and the rest off; and one reset.

use nonos_devmodel::FakeBar;

use super::state::Host;
use super::words::PANEL;
use crate::constants::{VG_MAX_SCANOUTS, VQ_STAGING_OFFSET};
use crate::tests::part::ring::{put, u32_at};

/// The last word of the used ring's page, which no driver writes: the host
/// marks its ring there, and a restart's wipe of the whole region clears it.
const MARK: usize = VQ_STAGING_OFFSET - 4;

impl Default for Host {
    fn default() -> Self {
        let mut displays = [(0, 0); VG_MAX_SCANOUTS];
        displays[0] = PANEL;
        let (scanouts, log, seen, used) = ([0; VG_MAX_SCANOUTS], Vec::new(), 0, 0);
        let (res, dead_at, dead_for, die_at) = (Default::default(), None, 0, None);
        let refuse_at = None;
        Host { res, scanouts, log, seen, used, dead_at, dead_for, die_at, refuse_at, displays }
    }
}

impl Host {
    /// A device reset (the wipe between holders, then the driver's own):
    /// every resource and scanout gone and a new ring, its displays kept.
    pub fn reset_device(&mut self) {
        *self = Host { displays: self.displays, ..Host::default() };
    }

    /// The driver's own reset, seen as its ring wiped (the host's mark gone):
    /// every resource and scanout gone and a new ring, while the log and the
    /// cues go on. The ring is marked again.
    pub fn marked(&mut self, bar: &FakeBar) {
        if u32_at(bar, MARK) != 0 {
            return;
        }
        let fresh = Host::default();
        (self.res, self.scanouts, self.seen, self.used) = (fresh.res, fresh.scanouts, 0, 0);
        put(bar, MARK, &1u32.to_le_bytes());
    }
}
