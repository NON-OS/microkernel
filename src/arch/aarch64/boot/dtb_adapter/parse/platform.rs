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

use super::describe;
use crate::arch::fdt::Fdt;

/// The devices a capsule may be handed, kept for the broker. A node that
/// cannot be read is left out by the finder; only a tree whose walk fails
/// refuses here, as it would at every other step.
pub(super) fn populate(fdt: &Fdt) -> Result<(), &'static [u8]> {
    crate::arch::aarch64::platform_devices::describe(fdt).map_err(describe)
}
