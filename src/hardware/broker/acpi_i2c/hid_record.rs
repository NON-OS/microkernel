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

use crate::arch::x86_64::acpi::aml::GpioController;
use crate::arch::x86_64::acpi::devices::i2c::{HidInterrupt, I2cHidDevice, I2cHostName};
use crate::hardware::broker::class::ids;
use crate::hardware::broker::device::{Bar, BarKind, BusKind, DeviceRecord};

/// Interrupt and descriptor facts carried in the I2C-HID record's bar 0
/// `flags` byte. The userland i2c drivers mirror these values.
pub(super) const HID_INFO_GPIO: u8 = 1 << 0;
pub(super) const HID_INFO_APIC: u8 = 1 << 1;
pub(super) const HID_INFO_LEVEL: u8 = 1 << 2;
pub(super) const HID_INFO_ACTIVE_HIGH: u8 = 1 << 3;
pub(super) const HID_INFO_DESC_FROM_DSM: u8 = 1 << 4;
pub(super) const HID_INFO_TEN_BIT: u8 = 1 << 5;

/// Build an ACPI-bus DeviceRecord for a firmware-declared I2C-HID device.
///
/// * `vendor`: 7-bit slave address, zero when the firmware fills it at run
///   time (the record then still names the controller the pad sits on).
/// * `device`: HID descriptor register (`_DSM` function 1, else 0x0001).
/// * `pci_progif`: the named host controller "I2Cn" as n+1, zero otherwise.
/// * `irq_pin`: GPIO community of a GpioInt as `_UID`+1, zero when unknown.
/// * `irq_source`: the GpioInt pin, or the GSI of an Interrupt resource.
/// * bar 0 (kind None, never a window; `bar_count` stays zero): `base` is
///   the MMIO base of a platform (non-PCI) host controller the ResourceSource
///   resolved to, `size` the controller NameSeg as a little-endian u32,
///   `aux` the ConnectionSpeed in Hz, `flags` the HID_INFO_* bits.
pub(super) fn hid_record(
    dev: &I2cHidDevice,
    gpio: &[GpioController],
    hosts: &[I2cHostName],
) -> DeviceRecord {
    let mut info = 0u8;
    let mut source = 0u32;
    let mut community = 0u8;
    let (level, active_high) = match dev.interrupt {
        HidInterrupt::Gpio { pin, level, active_high } => {
            info |= HID_INFO_GPIO;
            source = u32::from(pin);
            community = gpio_community_slot(dev.gpio_controller, gpio);
            (level, active_high)
        }
        HidInterrupt::Apic { gsi, level, active_high } => {
            info |= HID_INFO_APIC;
            source = gsi;
            (level, active_high)
        }
        HidInterrupt::None => (false, false),
    };
    if level {
        info |= HID_INFO_LEVEL;
    }
    if active_high {
        info |= HID_INFO_ACTIVE_HIGH;
    }
    if dev.desc_reg_from_dsm {
        info |= HID_INFO_DESC_FROM_DSM;
    }
    if dev.ten_bit {
        info |= HID_INFO_TEN_BIT;
    }
    let host_base = hosts.iter().find(|h| h.name == dev.controller).map_or(0, |h| h.mmio_base);
    let mut bars = [Bar::empty(); 6];
    bars[0] = Bar {
        base: host_base,
        size: u64::from(u32::from_le_bytes(dev.controller)),
        kind: BarKind::None as u8,
        flags: info,
        aux: dev.speed_hz,
        _pad: [0; 2],
    };
    DeviceRecord {
        bus_kind: BusKind::Acpi as u8,
        class: ids::I2C_HID,
        vendor: u16::from(dev.slave_addr),
        device: dev.hid_desc_reg,
        pci_progif: controller_slot(dev.controller),
        irq_pin: community,
        irq_source: source,
        bars,
        ..DeviceRecord::empty()
    }
}

/// Resolve the GpioInt ResourceSource NameSeg against the enumerated GPIO
/// community controllers and encode the match as `_UID`+1, zero when the
/// touchpad named no community, the name resolves to no enumerated controller,
/// or the index does not fit the field. The input driver uses it to select
/// which registered GPIO_CTRL record (matched on `device` == `_UID`) carries
/// the pad's interrupt pad.
fn gpio_community_slot(name: [u8; 4], gpio: &[GpioController]) -> u8 {
    if name == [0u8; 4] {
        return 0;
    }
    for ctl in gpio {
        if ctl.name == name {
            return match ctl.uid.checked_add(1) {
                Some(slot) if slot <= u32::from(u8::MAX) => slot as u8,
                _ => 0,
            };
        }
    }
    0
}

/// Encode the controller NameSeg "I2C0".."I2C9" as index+1, zero when absent
/// or not of that shape. Intel firmware names its LPSS I2C functions this way
/// in PCI function order, which is what lets the host driver map the name to
/// a PCI device id.
pub(super) fn controller_slot(name: [u8; 4]) -> u8 {
    if name[0..3] == *b"I2C" && name[3].is_ascii_digit() {
        name[3] - b'0' + 1
    } else {
        0
    }
}
