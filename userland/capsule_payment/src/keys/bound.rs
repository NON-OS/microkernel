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

//! The meter's root: the machine key under its caller-bound label
//! (`nonos_nox_meter::root`). Only this capsule's manifest namespace can
//! derive it; any other caller gets EPERM from the kernel.

use nonos_libc::machine_key;
use nonos_nox_meter::root::root_label;

pub fn root() -> Result<[u8; 32], i32> {
    machine_key(&root_label()).map_err(|e| e as i32)
}
