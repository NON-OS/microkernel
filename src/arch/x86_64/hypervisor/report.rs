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

/*
 * The `[PLATFORM]` boot line, after the broker has its device table: the
 * hypervisor, its Hyper-V interface, and a balloon left off.
 */

use super::balloon::balloon_offered;
use super::refine::refine;
use super::state;
use crate::sys::serial;

pub fn report() {
    let devices = crate::hardware::broker::list();
    let g = refine(state::guest(), devices.iter().map(|r| (r.vendor, r.device)));
    state::store(g);
    serial::print(b"[PLATFORM] hypervisor=");
    serial::print(g.vendor.slug().as_bytes());
    if g.hyperv_abi {
        serial::print(b" abi=hyperv");
    }
    if balloon_offered(devices.iter().map(|r| (r.vendor, r.device))) {
        serial::print(b" balloon=off");
    }
    serial::println(b"");
}
