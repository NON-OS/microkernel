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

/// Community windows a GPIO controller record can carry: the broker record
/// has six bars and the sixth names the `_HID`, and no Intel PCH has more
/// than five communities (Linux pinctrl-alderlake.c adls_communities).
pub const GPIO_MAX_WINDOWS: usize = 5;

/// A platform GPIO controller (Intel pinctrl or AMD FCH GPIO) enumerated
/// from the ACPI namespace by its `_HID`. A touchpad's GpioInt names one of
/// these as its ResourceSource; the i2c driver reads the pin's level through
/// the community window it falls in.
#[derive(Debug, Clone, Copy)]
pub struct GpioController {
    /// The device's own ACPI NameSeg (for example `GPO0`), matched against the
    /// trailing NameSeg of a GpioInt ResourceSource to pair a pin with its
    /// community.
    pub name: [u8; 4],
    /// The firmware `_UID`, the stable community index on a multi-community
    /// platform. Zero when the device declares none.
    pub uid: u32,
    /// The community windows as (base, length), in the order Linux numbers
    /// them (intel_community.barno), from `_CRS` when the firmware wrote
    /// them there.
    pub windows: [(u64, u64); GPIO_MAX_WINDOWS],
    pub window_count: usize,
    /// Sideband port ids of the communities, in the same order, for a PCH
    /// whose `_CRS` patches its windows in at run time from SBREG_BAR; empty
    /// when the windows are static.
    pub pids: &'static [u8],
    /// The eight-byte `_HID` that matched.
    pub hid: [u8; 8],
}

impl GpioController {
    pub fn new(hid: [u8; 8]) -> Self {
        let windows = [(0, 0); GPIO_MAX_WINDOWS];
        Self { name: [0; 4], uid: 0, windows, window_count: 0, pids: &[], hid }
    }

    /// Keep one window; false once the record is full.
    pub fn push_window(&mut self, base: u64, len: u64) -> bool {
        let Some(slot) = self.windows.get_mut(self.window_count) else { return false };
        *slot = (base, len);
        self.window_count += 1;
        true
    }

    /// Usable once a window is known, or can be computed from the sideband.
    pub fn is_valid(&self) -> bool {
        self.window_count > 0 || !self.pids.is_empty()
    }
}
