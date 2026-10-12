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

//! The devices a hypervisor emulates take the clients their physical
//! counterparts take: an input device the kernel alone (its driver posts
//! onto the input ring and serves nothing), a display the compositor alone,
//! a sound device the audio server alone.

use super::common::{owner, IPC};
use super::held::{callers_of, endpoint_admits, inbox_admits};

/// Every virtual input device's driver endpoint.
const VIRTUAL_INPUT: &[&str] = &["driver.virtio_input0", "driver.vmmdev0", "driver.vmmouse0"];

#[test]
fn a_virtual_pointer_or_keyboard_is_reached_by_no_capsule() {
    let everyone: &'static [&'static str] =
        &["compositor", "input_router", "wm", "app.terminal", "net.core", "driver.usb_hid0"];
    for &ep in VIRTUAL_INPUT {
        assert_eq!(callers_of(ep), Some(&[][..]), "{ep} names a caller");
        assert!(!endpoint_admits(ep, owner(everyone)), "{ep} took a capsule");
        assert!(!inbox_admits([(ep, IPC)], u64::MAX, owner(everyone)), "{ep} by pid");
    }
}

/// A virtual display adapter takes the compositor alone, as virtio-gpu does.
#[test]
fn a_virtual_display_takes_the_compositor_alone() {
    let compositor: &'static [&'static str] = &["compositor"];
    assert!(endpoint_admits("driver.vmware_svga0", owner(compositor)));
    for other in ["app.terminal", "wm", "input_router", "net.core", "driver.virtio_gpu0"] {
        let one: &'static [&'static str] = Box::leak(Box::new([other]));
        assert!(!endpoint_admits("driver.vmware_svga0", owner(one)), "took {other}");
    }
}

/// A virtual sound device takes the audio server alone, as HD Audio does:
/// an application reaches sound only through the server's mixer.
#[test]
fn a_virtual_sound_device_takes_the_audio_server_alone() {
    let server: &'static [&'static str] = &["audio.server"];
    assert!(endpoint_admits("driver.ac97_0", owner(server)));
    for other in ["app.audio_player", "app.terminal", "wm", "compositor", "driver.hda0"] {
        let one: &'static [&'static str] = Box::leak(Box::new([other]));
        assert!(!endpoint_admits("driver.ac97_0", owner(one)), "took {other}");
    }
}
