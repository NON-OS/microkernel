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

//! Devices whose claim is not open to whichever capsule asks first. Pure, so
//! the proofs run it on the host.
//!
//! - VirtualBox's VMMDev (80ee:cafe) is the guest's line to the host: mouse
//!   integration, display hints, host time, shared folders. Only the capsule
//!   spawned to serve it may claim it, so no other driver can speak to the
//!   host in its name.
//! - virtio-balloon (1af4:1002 transitional, 1af4:1045 modern; device type 5,
//!   Virtio 1.2, 5.5) hands guest memory to the host. Nothing claims it until
//!   the owner decides whether NONOS balloons at all (Decision 12) and the
//!   Balloon capability exists to say who may.

pub(crate) const VMMDEV_SERVICE: &str = "driver.vmmdev0";
pub(crate) const BOOTFB_SERVICE: &str = "driver.bootfb0";

/// The firmware bus's device number for the boot framebuffer (L31).
pub(crate) const FIRMWARE_BOOT_FRAMEBUFFER: u16 = 0x0001;

const VBOX_VENDOR: u16 = 0x80EE;
const VBOX_VMMDEV: u16 = 0xCAFE;
const VIRTIO_VENDOR: u16 = 0x1AF4;
const BALLOON_TRANSITIONAL: u16 = 0x1002;
const BALLOON_MODERN: u16 = 0x1045;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Hold {
    Open,
    /// Only the process that serves this endpoint may claim the device.
    ForService(&'static str),
    /// Only a process that serves one of these two may claim it.
    ForEither(&'static str, &'static str),
    Refused,
}

pub(crate) const fn hold_for(vendor: u16, device: u16) -> Hold {
    match (vendor, device) {
        (VBOX_VENDOR, VBOX_VMMDEV) => Hold::ForService(VMMDEV_SERVICE),
        (VIRTIO_VENDOR, BALLOON_TRANSITIONAL) | (VIRTIO_VENDOR, BALLOON_MODERN) => Hold::Refused,
        _ => Hold::Open,
    }
}

/// VMMDev reads the guest-physical addresses its requests carry directly,
/// outside any IOMMU the VM emulates (VB src/VBox/Devices/VMMDev/VMMDev.cpp
/// L3176, L3286, L3315). An IOVA handed to it would name the wrong memory, so
/// it is never attached to a domain and its grants are counted unconfined.
/// What firmware hands over at boot. The boot framebuffer shows whatever is
/// on screen, so only the process serving `driver.bootfb0` may hold it; no
/// other firmware device exists, and one that did is refused until named.
pub(crate) const fn hold_for_firmware(device: u16) -> Hold {
    match device {
        FIRMWARE_BOOT_FRAMEBUFFER => Hold::ForService(BOOTFB_SERVICE),
        _ => Hold::Refused,
    }
}

pub(crate) const fn untranslated(vendor: u16, device: u16) -> bool {
    vendor == VBOX_VENDOR && device == VBOX_VMMDEV
}
