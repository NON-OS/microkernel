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

use crate::orchestrator::enumerate::{enumerate, scan_hubs, Devices};

/// Look at every root port and every hub port again: a device plugged in
/// since is bound, one pulled out has its endpoints dropped.
pub(super) fn refresh_endpoints(xhci_port: u32, devs: &mut Devices, tries: &mut [u8; 256]) {
    enumerate(xhci_port, tries, devs);
    scan_hubs(xhci_port, devs);
}
