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

//! The trusted band's rules: only a trusted source places a layer there, its
//! holder is the owner of the topmost layer in it, and a raise never lifts a
//! client's layer into it.

use crate::scene::{Layer, SceneTable};
use crate::scene_raise::raise_by_pid;
use crate::trusted_rule::{holder, may_submit, TRUSTED_Z};

const SHELL: u32 = 7;
const APP: u32 = 40;

fn layer(owner_pid: u32, z: u32, stack: u32) -> Layer {
    let surface_handle = owner_pid as u64 * 100 + stack as u64;
    Layer {
        owner_pid,
        surface_handle,
        z,
        stack,
        width: 10,
        height: 10,
        in_use: true,
        ..Layer::default()
    }
}

#[test]
fn only_a_trusted_source_places_a_layer_in_the_band() {
    let running = [Some(SHELL)];
    assert!(may_submit(SHELL, TRUSTED_Z, &running));
    assert!(may_submit(SHELL, u32::MAX, &running));
    assert!(!may_submit(APP, TRUSTED_Z, &running), "a client is refused");
    assert!(!may_submit(APP, u32::MAX, &running));
    assert!(may_submit(APP, TRUSTED_Z - 1, &running), "under the band anyone may");
    assert!(!may_submit(SHELL, TRUSTED_Z, &[None]), "no shell running: nobody");
    assert!(!may_submit(0, TRUSTED_Z, &[Some(0)]), "pid 0 is never trusted");
}

#[test]
fn the_holder_is_the_owner_of_the_topmost_trusted_layer() {
    assert_eq!(holder(&[layer(APP, 3_000_000, 9), layer(SHELL, 1, 1)]), None);
    let up = [layer(APP, TRUSTED_Z - 1, 99), layer(SHELL, TRUSTED_Z, 1)];
    assert_eq!(holder(&up), Some(SHELL));
    let two = [layer(SHELL, TRUSTED_Z, 5), layer(8, TRUSTED_Z + 1, 1)];
    assert_eq!(holder(&two), Some(8), "the higher band wins over a later raise");
}

#[test]
fn a_raise_never_lifts_a_client_into_the_band() {
    let mut scene = SceneTable::new();
    scene.submit(layer(SHELL, TRUSTED_Z, 0)).unwrap();
    scene.submit(layer(APP, 2, 0)).unwrap();
    for _ in 0..5 {
        let _ = raise_by_pid(&mut scene, APP);
    }
    let app = scene.layers().find(|l| l.owner_pid == APP).unwrap();
    assert_eq!(app.z, 2, "a raise changes the order inside a band only");
    assert_eq!(holder(scene.layers()), Some(SHELL));
}
