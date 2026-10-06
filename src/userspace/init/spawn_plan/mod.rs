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

mod app_orchestrator;
mod apps;
mod apps_tools;
mod boot;
mod core;
#[cfg(not(feature = "microkernel-input-probe"))]
mod desktop_fleet;
#[cfg(not(feature = "microkernel-input-probe"))]
mod desktop_services;
// Every driver whose spawn asks it first: one left out here fails the build
// of any profile that carries that driver alone.
#[cfg(any(
    feature = "nonos-capsule-driver-ahci",
    feature = "nonos-capsule-driver-nvme",
    feature = "nonos-capsule-driver-virtio-blk",
    feature = "nonos-capsule-driver-virtio-net",
    feature = "nonos-capsule-driver-usb-msc",
    feature = "nonos-capsule-driver-iwlwifi",
    feature = "nonos-capsule-driver-rtl8821ce"
))]
mod device_present;
mod drivers_bus;
mod drivers_input;
mod drivers_nic;
mod drivers_storage;
mod drivers_usb;
mod drivers_virtio;
mod drivers_virtio_display;
mod drivers_virtio_io;
mod drivers_wifi;
#[cfg(feature = "microkernel-input-probe")]
mod input_probe_fleet;
#[cfg(feature = "microkernel-setup-wizard")]
mod install_first;
mod network;
mod orchestrator;
mod services_audio;
#[cfg(feature = "microkernel-setup-wizard")]
mod wizard_plan;
pub(super) use app_orchestrator::spawn_apps;
#[cfg(any(feature = "microkernel-input-probe", not(feature = "microkernel-setup-wizard")))]
pub(super) use orchestrator::spawn_desktop;
#[cfg(not(feature = "microkernel-setup-wizard"))]
pub(super) use orchestrator::spawn_market;
pub(super) use orchestrator::{
    spawn_core_after_ramfs, spawn_display_core, spawn_drivers, spawn_network, spawn_ramfs,
    spawn_vfs,
};

#[cfg(feature = "microkernel-setup-wizard")]
pub(super) use install_first::spawn_installer_first;
#[cfg(all(feature = "microkernel-setup-wizard", not(feature = "microkernel-input-probe")))]
pub(super) use wizard_plan::spawn_desktop;
#[cfg(feature = "microkernel-setup-wizard")]
pub(super) use wizard_plan::{spawn_market, spawn_post_wizard};
