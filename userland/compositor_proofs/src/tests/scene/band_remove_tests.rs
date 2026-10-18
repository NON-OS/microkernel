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

//! SCENE_REMOVE's band-only form: the sender's layer in that one band goes,
//! its other bands and every other owner's layers stay, and a band it holds
//! nothing in gives nothing back.

use crate::scene::{Layer, SceneTable};
use crate::trusted_rule::{holder, TRUSTED_Z};

const SHELL: u32 = 7;
const APP: u32 = 40;

fn layer(owner_pid: u32, z: u32, surface_handle: u64) -> Layer {
    Layer { owner_pid, surface_handle, z, width: 8, height: 8, in_use: true, ..Layer::default() }
}

#[test]
fn only_that_band_of_that_owner_goes() {
    let mut scene = SceneTable::new();
    for l in [layer(SHELL, 3_000_000, 5), layer(SHELL, TRUSTED_Z, 5), layer(APP, TRUSTED_Z - 1, 6)]
    {
        scene.submit(l).unwrap();
    }
    assert_eq!(holder(scene.layers()), Some(SHELL));
    let gone = scene.drop_band(SHELL, TRUSTED_Z).unwrap();
    assert_eq!((gone.owner_pid, gone.z, gone.surface_handle), (SHELL, TRUSTED_Z, 5));
    assert_eq!(holder(scene.layers()), None, "the prompt's band is empty again");
    let left: Vec<_> = scene.layers().map(|l| (l.owner_pid, l.z)).collect();
    assert_eq!(left.len(), 2);
    assert!(left.contains(&(SHELL, 3_000_000)) && left.contains(&(APP, TRUSTED_Z - 1)));
    assert!(scene.layers().any(|l| l.surface_handle == 5), "the surface is still shown");
}

#[test]
fn a_band_held_by_nobody_gives_nothing() {
    let mut scene = SceneTable::new();
    scene.submit(layer(APP, 2, 6)).unwrap();
    assert!(scene.drop_band(SHELL, 2).is_none(), "another owner's band");
    assert!(scene.drop_band(APP, 3).is_none(), "a band it holds nothing in");
    assert_eq!(scene.layers().count(), 1);
}
