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

// Every virtual machine driver whose spawn asks first: one left out here
// fails the build of a profile that carries that driver alone.
#[cfg(any(
    feature = "nonos-capsule-driver-pcnet",
    feature = "nonos-capsule-driver-vmxnet3",
    feature = "nonos-capsule-driver-virtio-input",
    feature = "nonos-capsule-driver-vmware-svga",
    feature = "nonos-capsule-driver-ac97",
    feature = "nonos-capsule-driver-vmmdev",
    feature = "nonos-capsule-driver-tulip",
    feature = "nonos-capsule-driver-piix-ide",
    feature = "nonos-capsule-driver-virtio-scsi",
    feature = "nonos-capsule-driver-pvscsi",
    feature = "nonos-capsule-driver-buslogic",
    feature = "nonos-capsule-driver-mpt"
))]
mod present;

mod audio;
mod bus;
mod display;
mod input;
mod nic;
mod storage;

pub(super) use audio::spawn as spawn_audio;
pub(super) use bus::spawn as spawn_bus;
pub(super) use display::spawn as spawn_display;
pub(super) use input::spawn as spawn_input;
pub(super) use nic::spawn as spawn_nics;
pub(super) use storage::spawn as spawn_storage;
