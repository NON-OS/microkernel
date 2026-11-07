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

pub(in crate::userspace::init) fn spawn_ramfs() {
    super::core::spawn_ramfs();
}

pub(in crate::userspace::init) fn spawn_core_after_ramfs() {
    super::core::spawn_after_ramfs();
}

#[cfg(not(feature = "microkernel-input-probe"))]
pub(in crate::userspace::init) fn spawn_display_core() {
    super::desktop_fleet::spawn_early_display();
}

#[cfg(feature = "microkernel-input-probe")]
pub(in crate::userspace::init) fn spawn_display_core() {}
/// The driver stages in the order their declared needs give
/// (`order::GRAPH`), the same on every boot.
pub(in crate::userspace::init) fn spawn_drivers() {
    crate::hardware::inventory::report::say();
    for stage in super::order::ORDER {
        spawn_stage(stage);
    }
    *LAST.lock() = Some(seen());
    crate::sys::bringup::summary("drivers");
}

/// The last walk's view; None until init's first walk is done.
static LAST: spin::Mutex<Option<super::late::Seen>> = spin::Mutex::new(None);

fn seen() -> super::late::Seen {
    super::late::Seen {
        pass_done: crate::hardware::broker::acpi::first_pass::done(),
        devices: crate::hardware::broker::list().len(),
    }
}

/// On init's tick: the stages after the ACPI stage walked again, in order,
/// when the capsule's first pass came or the device table grew since the
/// last walk (`late`).
pub(in crate::userspace::init) fn spawn_late() {
    let now = seen();
    {
        let mut last = LAST.lock();
        match *last {
            Some(before) if super::late::due(before, now) => *last = Some(now),
            _ => return,
        }
    }
    crate::sys::bringup::event("DRIVERS", crate::sys::bringup::Ev::Ready("a late publish"));
    for stage in super::order::ORDER.into_iter().filter(|s| super::order::after_acpi(*s)) {
        spawn_stage(stage);
    }
}

fn spawn_stage(stage: super::order::Stage) {
    use super::order::Stage::*;
    match stage {
        Virtio => super::drivers_virtio::spawn(),
        PlatformAcpi => super::platform_acpi::spawn(),
        VmBus => super::vm::spawn_bus(),
        VmDisplay => super::vm::spawn_display(),
        Disks => super::drivers_storage::spawn_disks(),
        UsbHosts => super::drivers_usb_hosts::spawn(),
        Buses => super::drivers_bus::spawn(),
        Input => super::drivers_input::spawn(),
        VmInput => super::vm::spawn_input(),
        Nics => super::drivers_nic::spawn(),
        VmNics => super::vm::spawn_nics(),
        UsbClass => super::drivers_usb::spawn(),
        UsbNet => super::drivers_usbnet::spawn(),
        Storage => super::drivers_storage::spawn(),
        VmStorage => super::vm::spawn_storage(),
        VmAudio => super::vm::spawn_audio(),
        Audio => super::services_audio::spawn(),
    }
}

pub(in crate::userspace::init) fn spawn_vfs() {
    super::core::spawn_vfs();
}

pub(in crate::userspace::init) fn spawn_network() {
    super::network::spawn();
}

#[cfg(feature = "microkernel-input-probe")]
pub(in crate::userspace::init) fn spawn_desktop() {
    super::input_probe_fleet::spawn();
}

#[cfg(all(not(feature = "microkernel-input-probe"), not(feature = "microkernel-setup-wizard")))]
pub(in crate::userspace::init) fn spawn_desktop() {
    super::desktop_fleet::spawn();
}

#[cfg(not(feature = "microkernel-setup-wizard"))]
pub(in crate::userspace::init) fn spawn_market() {
    super::core::spawn_market();
}
