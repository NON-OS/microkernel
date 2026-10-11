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

//! The sound devices a hypervisor emulates, each spawned only where the
//! device table has its device, and before the audio server, which looks
//! its sink up once at start.

pub(in crate::userspace::init::spawn_plan) fn spawn() {
    spawn_ac97();
}

#[cfg(feature = "nonos-capsule-driver-ac97")]
fn spawn_ac97() {
    use crate::hardware::ac97_capsule as c;
    use crate::hardware::inventory::HardwareFamily;
    if !super::present::present("DRIVER-AC97", HardwareFamily::AudioAc97) {
        return;
    }
    super::super::boot::capsule(
        "DRIVER-AC97",
        "driver_ac97",
        c::spawn_driver_ac97_capsule,
        c::shared_state,
    );
}
#[cfg(not(feature = "nonos-capsule-driver-ac97"))]
fn spawn_ac97() {}
