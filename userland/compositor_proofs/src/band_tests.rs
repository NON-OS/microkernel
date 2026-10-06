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

//! A process holds one layer per z band. The desktop shell keeps its desktop
//! in the band under the windows and its menubar, dock, menus and dialogs in
//! one over them; each window is one layer in the band between.

use crate::frame_model::layer;
use crate::scene::SceneTable;
use crate::scene_raise::raise_by_pid;
use crate::scene_submit::submit_layer;

const SHELL: u32 = 7;
const DESK_Z: u32 = 1;
const APP_Z: u32 = 2;
const SHELL_TOP_Z: u32 = 3_000_000;
/// scene/layer.rs MAX_LAYERS_PER_OWNER.
const MAX_LAYERS_PER_OWNER: u32 = 4;

fn order(scene: &SceneTable) -> Vec<(u32, u32)> {
    let (layers, n) = scene.z_sorted_snapshot();
    layers[..n].iter().map(|l| (l.owner_pid, l.z)).collect()
}

fn desktop_with_two_windows() -> SceneTable {
    let mut scene = SceneTable::new();
    submit_layer(&mut scene, layer(SHELL, 0, 0, 100, 100, DESK_Z)).expect("desk");
    submit_layer(&mut scene, layer(SHELL, 0, 0, 100, 100, SHELL_TOP_Z)).expect("chrome");
    submit_layer(&mut scene, layer(1, 10, 10, 50, 50, APP_Z)).expect("window 1");
    submit_layer(&mut scene, layer(2, 20, 20, 50, 50, APP_Z)).expect("window 2");
    scene
}

#[test]
fn the_shells_chrome_draws_over_every_window_and_its_desk_under_them() {
    let scene = desktop_with_two_windows();
    assert_eq!(scene.layers().count(), 4);
    assert_eq!(order(&scene), vec![(SHELL, DESK_Z), (1, APP_Z), (2, APP_Z), (SHELL, SHELL_TOP_Z)]);
}

#[test]
fn a_resubmit_replaces_only_its_own_bands_layer() {
    let mut scene = desktop_with_two_windows();
    let moved = layer(SHELL, 0, 0, 100, 100, SHELL_TOP_Z);
    let moved = crate::scene::Layer { surface_handle: 99, ..moved };
    submit_layer(&mut scene, moved).expect("resubmit");
    assert_eq!(scene.layers().count(), 4);
    let desk = scene.layers().find(|l| l.owner_pid == SHELL && l.z == DESK_Z).expect("desk kept");
    assert_ne!(desk.surface_handle, 99);
    let top = scene.layers().find(|l| l.owner_pid == SHELL && l.z == SHELL_TOP_Z).expect("top");
    assert_eq!(top.surface_handle, 99);
}

#[test]
fn raising_a_window_never_lifts_it_over_the_shells_chrome() {
    let mut scene = desktop_with_two_windows();
    raise_by_pid(&mut scene, 1);
    assert_eq!(order(&scene), vec![(SHELL, DESK_Z), (2, APP_Z), (1, APP_Z), (SHELL, SHELL_TOP_Z)]);
    // Raising the shell (its dock was clicked) lifts neither of its layers
    // out of its band: the desk stays under the windows.
    raise_by_pid(&mut scene, SHELL);
    assert_eq!(order(&scene), vec![(SHELL, DESK_Z), (2, APP_Z), (1, APP_Z), (SHELL, SHELL_TOP_Z)]);
}

#[test]
fn one_process_cannot_fill_the_table() {
    let mut scene = SceneTable::new();
    for band in 0..MAX_LAYERS_PER_OWNER {
        submit_layer(&mut scene, layer(5, 0, 0, 10, 10, band + 1)).expect("under the cap");
    }
    assert!(submit_layer(&mut scene, layer(5, 0, 0, 10, 10, 99)).is_err());
    // Its existing bands still take resubmits, and other processes still fit.
    submit_layer(&mut scene, layer(5, 1, 1, 10, 10, 1)).expect("a move in a held band");
    submit_layer(&mut scene, layer(6, 0, 0, 10, 10, 2)).expect("another process");
}
