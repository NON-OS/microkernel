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

//! The census's facts on this machine: the firmware tables' declarations,
//! the ACPI capsule's latest report, the lid, and each device in the
//! broker's table with whether a driver holds it.

use super::rule::{Battery, Class, Count, Declared, Facts};
use crate::hardware::broker::acpi::{lid_state, power_report as pr};
use crate::hardware::broker::claim::claimed;
use crate::hardware::broker::list;
use crate::hardware::inventory::{
    classify_device, family_driver, missing_path, present, HardwareFamily as F,
};

/// The census class of a device-table family; None for those it leaves to
/// another row (USB storage is the USB host's) or does not count (bridges,
/// SMBus, SPI and I2C hosts, which the pointer row stands for).
fn class_of(f: F) -> Option<Class> {
    use F::*;
    Some(match f {
        StorageNvme | StorageAhci | StorageVirtioBlk | StorageVirtioScsi | StorageBuslogic
        | StorageMpt | StorageVmwarePvscsi | StorageVmd | StorageEmmc | StorageSdhci
        | StorageUfs | StorageIde => Class::Storage,
        NetworkVirtio | NetworkE1000 | NetworkE1000e | NetworkIgb | NetworkIgc | NetworkIxgbe
        | NetworkRtl8139 | NetworkRtl8169 | NetworkPcnet | NetworkVmxnet3 | NetworkTulip
        | NetworkRtl8821ce | NetworkRtl8822ce | NetworkRtl8852be | NetworkIwlwifi
        | NetworkRealtekWifi | NetworkMt7921 => Class::Network,
        DisplayVirtioGpu | DisplayBga | DisplayVmwareSvga | DisplayNativeIntel
        | DisplayNativeAmd | DisplayNativeNvidia => Class::Display,
        UsbXhci | UsbEhci | UsbOhci | UsbUhci => Class::Usb,
        AudioHda | AudioAc97 => Class::Audio,
        InputPs2 | InputVirtio => Class::Keyboard,
        InputI2cHid | InputVmmdev => Class::Pointer,
        _ => return None,
    })
}

fn add(c: &mut Count, f: F, held: bool) {
    let held_why =
        missing_path(f).or_else(|| family_driver(f).is_none().then_some("no driver here"));
    match held_why {
        Some(why) => {
            c.held = c.held.saturating_add(1);
            c.held_why.get_or_insert(why);
        }
        None => {
            c.expected = c.expected.saturating_add(1);
            if held {
                c.up = c.up.saturating_add(1);
            } else {
                c.unclaimed_by = c.unclaimed_by.or(family_driver(f));
            }
        }
    }
}

fn battery(word: Option<u64>) -> Battery {
    match word {
        None => Battery::Unreported,
        Some(w) if w & pr::BATTERY_PRESENT == 0 => Battery::Empty,
        Some(w) if w & pr::PERCENT_MASK <= 100 => Battery::Charged,
        Some(_) => Battery::Unread,
    }
}

#[cfg(feature = "nonos-capsule-acpi")]
fn acpi_capsule() -> bool {
    crate::userspace::capsule_acpi::shared_state().is_alive()
}

#[cfg(not(feature = "nonos-capsule-acpi"))]
fn acpi_capsule() -> bool {
    false
}

pub(super) fn facts(now_ms: u64) -> Facts {
    let word = pr::state(now_ms);
    let acpi = crate::arch::acpi_declared().map(|(battery, ac, lid, thermal_zones)| Declared {
        battery,
        ac,
        lid,
        thermal_zones,
    });
    let mut f = Facts {
        acpi,
        acpi_capsule: acpi_capsule(),
        battery: battery(word),
        ac_known: word.is_some_and(|w| w & pr::AC_KNOWN != 0),
        lid_known: lid_state::known(),
        boot_framebuffer: present(F::DisplayGopFramebuffer),
        ..Facts::default()
    };
    for rec in list() {
        let family = classify_device(&rec);
        let held = claimed(rec.device_id);
        let c = match class_of(family) {
            Some(Class::Keyboard) => &mut f.keyboard,
            Some(Class::Pointer) => &mut f.pointer,
            Some(Class::Display) => &mut f.display,
            Some(Class::Storage) => &mut f.storage,
            Some(Class::Network) => &mut f.network,
            Some(Class::Usb) => &mut f.usb,
            Some(Class::Audio) => &mut f.audio,
            _ => continue,
        };
        add(c, family, held);
    }
    f
}
