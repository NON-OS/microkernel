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

//! The rows of scope_table.rs with more than one entry.

use super::lpss::LPSS_I2C;
use super::rule::Match::{self, Acpi, Class, Ids};

pub(super) const INTEL: u16 = 0x8086;
pub(super) const VMWARE: u16 = 0x15AD;
pub(super) const VBOX: u16 = 0x80EE;
pub(super) const REALTEK: u16 = 0x10EC;
pub(super) const VIRTIO: u16 = 0x1AF4;
pub(super) const ANY: Option<u16> = None;

/// SATA from any vendor, and Intel RST RAID.
pub(super) const AHCI: &[Match] = &[
    Class { vendor: ANY, class: 0x01, sub: 0x06, progif: None },
    Class { vendor: Some(INTEL), class: 0x01, sub: 0x04, progif: None },
];

/// Intel LPSS I2C by id, any other Intel serial-bus or signal-processing
/// function the capsule takes as unlisted LPSS, and the ACPI-declared I2C
/// controllers (SERIAL). The GPIO communities are gpio0's: the touchpad's
/// doorbell asks driver.gpio0 and never claims one.
pub(super) const I2C_PCI: &[Match] = &[
    Ids(INTEL, LPSS_I2C),
    Class { vendor: Some(INTEL), class: 0x0C, sub: 0x80, progif: None },
    Class { vendor: Some(INTEL), class: 0x11, sub: 0x80, progif: None },
    Acpi { class: 0x0060, vendor: ANY, device: ANY },
];

/// The synthesized PS/2 keyboard and aux port (broker/platform.rs).
pub(super) const PS2: &[Match] = &[
    Acpi { class: 0x0040, vendor: Some(1), device: Some(0x0303) },
    Acpi { class: 0x0040, vendor: Some(1), device: Some(0x0304) },
];

/// HD Audio from any vendor, and Intel's audio-class controllers.
pub(super) const HDA: &[Match] = &[
    Class { vendor: ANY, class: 0x04, sub: 0x03, progif: None },
    Class { vendor: Some(INTEL), class: 0x04, sub: 0x01, progif: None },
];

/// SVGA II as VMware lays it out, and VirtualBox's VBoxSVGA.
pub(super) const SVGA: &[Match] = &[Ids(VMWARE, &[0x0405, 0x0405]), Ids(VBOX, &[0xBEEF, 0xBEEF])];

/// Intel graphics, VGA-compatible when firmware made it the boot display and
/// "other display" when it did not; the capsule's id tables pick the
/// generations it drives.
pub(super) const INTEL_GFX: &[Match] = &[
    Class { vendor: Some(INTEL), class: 0x03, sub: 0x00, progif: None },
    Class { vendor: Some(INTEL), class: 0x03, sub: 0x80, progif: None },
];
