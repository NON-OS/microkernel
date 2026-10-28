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

//! Safe Mode is no audio: every audio capsule the kernel starts, playback
//! or capture, is on the spawn gate's NOT_SAFE list under the name its
//! spawn registers, and the list is what Safe Mode refuses.

const REFUSE: &str = include_str!(
    "../../../../src/kernel_core/process_spawn/capsule_spawn/runner/profile_refuse.rs"
);

/// Each audio capsule's spawn: its SERVICE_NAME is the name the gate sees.
const AUDIO: [&str; 5] = [
    include_str!("../../../../src/hardware/hda_capsule/spawn.rs"),
    include_str!("../../../../src/hardware/ac97_capsule/spawn.rs"),
    include_str!("../../../../src/hardware/usb_audio_capsule/spawn.rs"),
    include_str!("../../../../src/hardware/usb_mic_capsule/spawn.rs"),
    include_str!("../../../../src/hardware/audio_capsule/spawn.rs"),
];

/// The names between `const NOT_SAFE` and the list's close.
fn not_safe() -> Vec<&'static str> {
    let list = REFUSE.split("const NOT_SAFE").nth(1).expect("NOT_SAFE");
    let list = list.split("];").next().expect("closed");
    let body = list.split('=').nth(1).expect("a value");
    body.split('"').skip(1).step_by(2).collect()
}

fn service_name(spawn: &str) -> &str {
    let rest = spawn.split("const SERVICE_NAME: &str = \"").nth(1).expect("SERVICE_NAME");
    rest.split('"').next().expect("closed")
}

#[test]
fn every_audio_capsule_is_refused_in_safe_mode() {
    let list = not_safe();
    for spawn in AUDIO {
        let name = service_name(spawn);
        assert!(list.contains(&name), "{name} starts in Safe Mode");
    }
    assert!(list.contains(&"driver.usb_audio0") && list.contains(&"driver.usb_mic0"));
}

#[test]
fn the_list_is_what_safe_mode_refuses() {
    assert!(REFUSE.contains("let not_safe = NOT_SAFE.iter().any(|n| is_instance(name, n));"));
    assert!(REFUSE.contains("(profile.minimal() && not_safe)"));
    let profile = include_str!("../../../../src/boot/handoff/api/profile.rs");
    let minimal = profile.split("pub const fn minimal(self)").nth(1).expect("minimal");
    assert!(minimal.split('}').next().unwrap().contains("matches!(self, Self::Safe)"));
}
