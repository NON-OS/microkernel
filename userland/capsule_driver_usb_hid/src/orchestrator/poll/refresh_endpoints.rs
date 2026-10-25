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

use crate::orchestrator::enumerate::enumerate;
use crate::xhci::{host, Hosts};

use super::held::Held;

/// Look at every port of every host again, root and virtual: a host
/// registered since is searched, a device plugged in since is bound, one
/// pulled out has its endpoints dropped.
pub(super) fn refresh_endpoints(hosts: &mut Hosts<Held>) {
    hosts.find(host);
    hosts.each(|xhci_port, held| enumerate(xhci_port, &mut held.roots, &mut held.devs));
}
