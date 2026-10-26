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

//! The display drivers: every name the compositor tries is held to it, and
//! every display driver the kernel spawns is one of those names.

use super::common::owner;
use super::display_hints::HINT_SOURCES;
use super::held::displays::DISPLAYS;
use super::held::{callers_of, endpoint_admits};

#[test]
fn every_display_is_held_to_the_compositor_alone() {
    for ep in DISPLAYS {
        assert_eq!(callers_of(ep), Some(&["compositor"][..]), "{ep}");
        assert!(endpoint_admits(ep, owner(&["compositor"])), "{ep} refused the compositor");
        for other in ["app.about", "app.browser", "wm", "input_router", "audio.server"] {
            let one: &'static [&'static str] = Box::leak(Box::new([other]));
            assert!(!endpoint_admits(ep, owner(one)), "{ep} took {other}");
        }
    }
    assert!(
        !DISPLAYS.contains(&"driver.virtio_net0") && callers_of("driver.virtio_net0").is_some()
    );
}

/// The boot framebuffer is the fallback and goes last whenever it is listed;
/// no name is listed twice; no host hints are taken until the owner says.
#[test]
fn the_list_is_ordered_and_hints_are_off() {
    let last = DISPLAYS.iter().position(|d| *d == "driver.bootfb0");
    assert!(last.is_none_or(|i| i == DISPLAYS.len() - 1));
    let mut names = DISPLAYS.to_vec();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), DISPLAYS.len());
    assert!(HINT_SOURCES.is_empty());
}

/// Each display driver the kernel spawns registers a name in the list.
#[test]
fn every_spawned_display_name_is_listed() {
    let spawns = [include_str!("../../../../src/hardware/virtio_gpu_capsule/spawn.rs")];
    for src in spawns {
        let line = src.lines().find(|l| l.contains("const SERVICE_NAME")).expect("a name");
        let name = line.split('"').nth(1).expect("a quoted name");
        assert!(DISPLAYS.contains(&name), "{name} is spawned but not in displays.rs");
    }
}
