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

use nonos_libc::mk_battery_status;

use super::battery_text::Battery;

/// What the kernel says about the battery (`MkBatteryStatus`,
/// `src/syscall/microkernel/battery.rs`): a percent, no battery at all (the
/// firmware declares none: a desktop), or a battery whose charge cannot be
/// read. Today the last is every laptop, because reading `_BST` needs an AML
/// interpreter the kernel does not have; the bar says so in words instead of
/// drawing a gauge.
pub fn percent() -> Battery {
    Battery::from_status(mk_battery_status())
}
