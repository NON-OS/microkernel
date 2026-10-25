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

//! The network cards hypervisors emulate and no current machine carries,
//! each spawned only where the device table has its device (Hyper-V's,
//! where its SynIC is up).

pub(in crate::userspace::init::spawn_plan) fn spawn() {
    spawn_pcnet();
    spawn_vmxnet3();
    spawn_tulip();
    spawn_netvsc();
}

#[cfg(feature = "nonos-capsule-driver-pcnet")]
fn spawn_pcnet() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::pcnet_capsule as c;
    if !super::present::present("DRIVER-PCNET", HardwareFamily::NetworkPcnet) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-PCNET",
        "driver_pcnet",
        c::spawn_driver_pcnet_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-pcnet"))]
fn spawn_pcnet() {}

#[cfg(feature = "nonos-capsule-driver-vmxnet3")]
fn spawn_vmxnet3() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::vmxnet3_capsule as c;
    if !super::present::present("DRIVER-VMXNET3", HardwareFamily::NetworkVmxnet3) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-VMXNET3",
        "driver_vmxnet3",
        c::spawn_driver_vmxnet3_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-vmxnet3"))]
fn spawn_vmxnet3() {}

#[cfg(feature = "nonos-capsule-driver-tulip")]
fn spawn_tulip() {
    use crate::hardware::inventory::HardwareFamily;
    use crate::hardware::tulip_capsule as c;
    if !super::present::present("DRIVER-TULIP", HardwareFamily::NetworkTulip) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-TULIP",
        "driver_tulip",
        c::spawn_driver_tulip_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-tulip"))]
fn spawn_tulip() {}

/// Where the Hyper-V SynIC is up: the driver waits for the VMBus core to
/// give it the network adapter's channel, and leaves if none comes.
#[cfg(feature = "nonos-capsule-driver-netvsc")]
fn spawn_netvsc() {
    use crate::hardware::netvsc_capsule as c;
    if !crate::arch::hv_vmbus_ready() {
        crate::sys::boot_log::ok("DRIVER-NETVSC", "no Hyper-V SynIC, not spawned");
        return;
    }
    super::super::boot::capsule(
        "DRIVER-NETVSC",
        "driver_netvsc",
        c::spawn_driver_netvsc_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-netvsc"))]
fn spawn_netvsc() {}
