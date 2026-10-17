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
//! command, its displays scanout 0 at PANEL and the rest off.

use super::state::Host;
use super::words::PANEL;
use crate::constants::VG_MAX_SCANOUTS;

impl Default for Host {
    fn default() -> Self {
        let mut displays = [(0, 0); VG_MAX_SCANOUTS];
        displays[0] = PANEL;
        let (scanouts, log, seen, used) = ([0; VG_MAX_SCANOUTS], Vec::new(), 0, 0);
        let (res, dead_at, dead_for, die_at) = (Default::default(), None, 0, None);
        Host { res, scanouts, log, seen, used, dead_at, dead_for, die_at, displays }
    }
}
