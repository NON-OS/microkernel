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
    spawn_usb_audio();
    spawn_audio();
}

/// The USB audio driver, wherever a USB host is: whether a headset
/// is plugged in is known only once it searches the ports, so it starts
/// before the audio server, which asks it whether it has a device.
#[cfg(feature = "nonos-capsule-driver-usb-audio")]
fn spawn_usb_audio() {
    use crate::hardware::usb_audio_capsule as c;
    if !super::usb_host::usb_host() {
        crate::sys::boot_log::ok("DRIVER-USB-AUDIO", "no USB host, not spawned");
        return;
    }
    let spawn = c::spawn_driver_usb_audio_capsule;
    super::boot::capsule("DRIVER-USB-AUDIO", "driver_usb_audio", spawn, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-driver-usb-audio"))]
fn spawn_usb_audio() {}

#[cfg(feature = "nonos-capsule-audio")]
fn spawn_audio() {
    use crate::hardware::audio_capsule as c;
    super::boot::capsule("AUDIO", "audio_server", c::spawn_audio_capsule, c::shared_state);
}

#[cfg(not(feature = "nonos-capsule-audio"))]
fn spawn_audio() {}
