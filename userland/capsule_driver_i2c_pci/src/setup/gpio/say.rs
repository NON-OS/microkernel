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

use nonos_libc::mk_debug;

use crate::discover::AcpiTouchpad;
use crate::driver::Doorbell;

/// One console line on how the touchpad's interrupt line will be read, or
/// where finding its register stopped. Returns None for the caller to pass
/// on, as a refusal means the HID driver polls.
pub(super) fn say(tp: &AcpiTouchpad, what: &str) -> Option<Doorbell> {
    let line = alloc::format!(
        "driver.i2c_pci: touchpad interrupt line, GPIO pin {} of controller uid {}: {}\n",
        tp.gpio_pin,
        tp.gpio_community.saturating_sub(1),
        what
    );
    let _ = mk_debug(line.as_ptr(), line.len());
    None
}
