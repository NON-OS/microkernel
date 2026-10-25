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

pub(super) fn spawn() {
    spawn_gpu();
    spawn_bga();
    super::drivers_native_display::spawn();
    spawn_net();
}

/// The Bochs adapter, only where the inventory saw one: it is a display
/// driver, beside virtio-gpu, though not a virtio one.
#[cfg(feature = "nonos-capsule-driver-bga")]
fn spawn_bga() {
    use crate::hardware::bga_capsule as c;
    use crate::hardware::inventory::{present, HardwareFamily};
    if !present(HardwareFamily::DisplayBga) {
        crate::sys::boot_log::ok("DRIVER-BGA", "no controller present, not spawned");
        return;
    }
    super::boot::capsule("DRIVER-BGA", "driver_bga", c::spawn_driver_bga_capsule, c::shared_state);
}
#[cfg(not(feature = "nonos-capsule-driver-bga"))]
fn spawn_bga() {}

#[cfg(feature = "nonos-capsule-driver-virtio-gpu")]
fn spawn_gpu() {
    use crate::hardware::virtio_gpu_capsule as c;
    super::boot::capsule(
        "DRIVER-VIRTIO-GPU",
        "driver_virtio_gpu",
        c::spawn_driver_virtio_gpu_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-virtio-gpu"))]
fn spawn_gpu() {}

#[cfg(feature = "nonos-capsule-driver-virtio-net")]
fn spawn_net() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::virtio_net_capsule as c;
    if !super::device_present::present("DRIVER-VIRTIO-NET", HardwareFamily::NetworkVirtio) {
        return;
    }
    super::boot::capsule(
        "DRIVER-VIRTIO-NET",
        "driver_virtio_net",
        c::spawn_driver_virtio_net_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-virtio-net"))]
fn spawn_net() {}
