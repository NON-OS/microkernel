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

//! Damage wholly under an opaque layer over it is not composed
//! (state/hidden.rs), and no pixel is left stale by that. Windows here draw
//! new frames, some opaque and some not, commit damage and present as the
//! handlers do, move, are raised and leave; the screen, kept only through
//! the damage taken, must equal a full frame after every step; some move to
//! another band as the shell moves its chrome, a twin placed in the new band
//! and the old one dropped (state/twin.rs), checked between the two steps
//! too. And the lock screen's case: the Terminal's uptime row under the
//! curtain composes nothing, the curtain going up and coming down composes
//! the screen once each, and the screen is right once it lifts.

use crate::damage::{DamageAccumulator, Rect};
use crate::hidden::{shown_by_owner, shown_on};
use crate::scene::{Layer, SceneTable};
use crate::scene_raise::raise_by_pid;
use crate::scene_remove::remove_by_pid;
use crate::scene_submit::{rect_of, submit_layer};
use crate::state::present_rule::present;
use crate::state::twin;
use crate::tests::scene::frame_model::{layer, Rng, H, W};

const Z: u32 = 2;
const TRUSTED_Z: u32 = 0x4000_0000;
/// Three bands a layer may be in; one between two others.
const BANDS: [u32; 3] = [Z, 3_000_000, TRUSTED_Z];

/// Each pixel: the layers that show there, top down to the first opaque one,
/// each its owner, its frame count and whether it is opaque, folded; what a
/// translucent layer over another shows depends on both.
fn full_px(scene: &SceneTable, frames: &[u32], x: u32, y: u32) -> u64 {
    let (layers, n) = scene.z_sorted_snapshot();
    let mut v: u64 = 0xcbf2_9ce4_8422_2325;
    for l in layers[..n].iter().rev() {
        if x < l.x || x - l.x >= l.width || y < l.y || y - l.y >= l.height {
            continue;
        }
        let f = frames[l.owner_pid as usize] as u64;
        v = (v ^ ((l.owner_pid as u64) << 40 | f << 1 | l.opaque as u64))
            .wrapping_mul(0x100_0000_01b3);
        if l.opaque {
            return v;
        }
    }
    v
}

fn compose(scene: &SceneTable, frames: &[u32], px: &mut [u64], clip: Rect) {
    for y in clip.y..clip.bottom().min(H) {
        for x in clip.x..clip.right().min(W) {
            px[(y * W + x) as usize] = full_px(scene, frames, x, y);
        }
    }
}

fn full(scene: &SceneTable, frames: &[u32]) -> Vec<u64> {
    let mut px = vec![0u64; (W * H) as usize];
    compose(scene, frames, &mut px, Rect { x: 0, y: 0, width: W, height: H });
    px
}

/// DAMAGE_COMMIT's handler: clipped to the canvas, bounded, the hidden
/// left, and the layers drawn in no longer taken as opaque.
fn commit(scene: &mut SceneTable, acc: &mut DamageAccumulator, r: Rect, owner: u32) {
    if r.is_empty() || r.x >= W || r.y >= H {
        return;
    }
    let r = Rect { width: r.width.min(W - r.x), height: r.height.min(H - r.y), ..r };
    for part in shown_by_owner(scene, r, owner) {
        acc.accumulate(part);
    }
    scene.drew(owner, |l| r.intersect(rect_of(l)).is_some());
}

/// SCENE_SUBMIT's handler: a fresh twin of an opaque layer damages only what
/// it changes, anything else its repaint.
fn submit_as(scene: &mut SceneTable, acc: &mut DamageAccumulator, l: Layer) {
    let fresh = !scene.layers().any(|x| !x.keyed && x.owner_pid == l.owner_pid && x.z == l.z);
    let Ok(rs) = submit_layer(scene, l) else { return };
    if fresh && twin::placed(scene, acc, (l.owner_pid, l.z, l.surface_handle)) {
        return;
    }
    rs.into_iter().flatten().for_each(|r| acc.accumulate(r));
}

/// SCENE_REMOVE with BAND_ONLY's handler.
fn drop_band_as(scene: &mut SceneTable, acc: &mut DamageAccumulator, owner: u32, z: u32) {
    let Some(gone) = scene.drop_band(owner, z) else { return };
    if !twin::dropped(scene, acc, &gone) {
        acc.accumulate(rect_of(&gone));
    }
}

/// OP_PRESENT's handler for a layer's own buffer: its opacity kept, the
/// changed part repainted unless hidden.
fn present_as(
    scene: &mut SceneTable,
    acc: &mut DamageAccumulator,
    l: &Layer,
    changed: Rect,
    opaque: bool,
) {
    let Some(s) = present(scene, l.owner_pid, l.z, l.surface_handle, changed) else {
        return;
    };
    assert!(scene.set_opaque(l.owner_pid, l.surface_handle, opaque), "its layer");
    let on = (l.owner_pid, l.z, l.surface_handle);
    if let Some(r) = s.repaint.and_then(|r| shown_on(scene, on, r)) {
        acc.accumulate(r);
    }
}

fn settle(
    scene: &SceneTable,
    acc: &mut DamageAccumulator,
    frames: &[u32],
    screen: &mut [u64],
) -> u64 {
    let mut composed = 0;
    while let Some(r) = acc.drain() {
        composed += r.area();
        compose(scene, frames, screen, r);
    }
    composed
}

#[test]
fn hidden_damage_never_leaves_a_stale_pixel() {
    let mut culled = 0u64;
    for seed in 1..=96u64 {
        let mut rng = Rng::new(seed);
        let (mut scene, mut acc, mut frames) =
            (SceneTable::new(), DamageAccumulator::new(), [0u32; 7]);
        let mut screen = full(&scene, &frames);
        for step in 0..160 {
            let k = 1 + rng.below(6);
            let was = scene.layers().find(|l| l.owner_pid == k).copied();
            let (x, y, w, h) = (rng.below(W), rng.below(H), 1 + rng.below(40), 1 + rng.below(30));
            match (rng.below(7), was) {
                (0, _) | (_, None) => {
                    let z =
                        was.map_or(if k == 6 { TRUSTED_Z } else { BANDS[k as usize % 3] }, |o| o.z);
                    submit_as(&mut scene, &mut acc, layer(k, x, y, w, h, z));
                }
                (6, Some(o)) => {
                    // To another band as a twin, then out of the old one.
                    let to = BANDS[(BANDS.iter().position(|&b| b == o.z).unwrap_or(0)
                        + 1
                        + rng.below(2) as usize)
                        % 3];
                    submit_as(&mut scene, &mut acc, Layer { z: to, stack: 0, opaque: false, ..o });
                    settle(&scene, &mut acc, &frames, &mut screen);
                    assert!(
                        screen == full(&scene, &frames),
                        "seed {seed} step {step}: a stale pixel with both twins"
                    );
                    drop_band_as(&mut scene, &mut acc, k, o.z);
                }
                (1, Some(o)) => {
                    // A new frame, every pixel of it changed, said opaque or not.
                    frames[k as usize] += 1;
                    let whole = Rect { x: 0, y: 0, width: o.width, height: o.height };
                    present_as(&mut scene, &mut acc, &o, whole, rng.below(3) != 0);
                }
                (2, Some(o)) => {
                    frames[k as usize] += 1;
                    let shown: u64 = shown_by_owner(&scene, rect_of(&o), k).map(|r| r.area()).sum();
                    culled += rect_of(&o).area() - shown;
                    commit(&mut scene, &mut acc, rect_of(&o), k);
                }
                (3, Some(_)) => {
                    if let Some(r) = raise_by_pid(&mut scene, k) {
                        acc.accumulate(r);
                    }
                }
                (4, Some(_)) => acc.accumulate(remove_by_pid(&mut scene, k).unwrap_or_default()),
                _ => commit(&mut scene, &mut acc, Rect { x: 0, y: 0, width: W, height: H }, k),
            }
            settle(&scene, &mut acc, &frames, &mut screen);
            assert!(screen == full(&scene, &frames), "seed {seed} step {step}: a stale pixel");
        }
    }
    assert!(culled > 0, "some damage was hidden, so the proof reached the cull");
}

const SHELL: u32 = 1;
const TERMINAL: u32 = 2;
const SCREEN: Rect = Rect { x: 0, y: 0, width: W, height: H };
/// The Terminal's uptime row, a few pixels of its banner.
const ROW: Rect = Rect { x: 4, y: 3, width: 30, height: 2 };

fn locked() -> (SceneTable, DamageAccumulator, [u32; 3], Vec<u64>) {
    let (mut scene, mut acc, frames) = (SceneTable::new(), DamageAccumulator::new(), [0u32; 3]);
    let terminal = layer(TERMINAL, 2, 2, 40, 30, Z);
    submit_layer(&mut scene, terminal).expect("room");
    let curtain = layer(SHELL, 0, 0, W, H, TRUSTED_Z);
    submit_layer(&mut scene, curtain).expect("room");
    let mut screen = full(&scene, &frames);
    present_as(&mut scene, &mut acc, &curtain, SCREEN, true);
    settle(&scene, &mut acc, &frames, &mut screen);
    (scene, acc, frames, screen)
}

#[test]
fn the_terminal_under_the_curtain_composes_nothing() {
    let (mut scene, mut acc, mut frames, mut screen) = locked();
    let terminal = *scene.layers().find(|l| l.owner_pid == TERMINAL).expect("terminal");
    for _ in 0..5 {
        frames[TERMINAL as usize] += 1;
        commit(
            &mut scene,
            &mut acc,
            Rect { x: terminal.x + ROW.x, y: terminal.y + ROW.y, ..ROW },
            TERMINAL,
        );
        present_as(&mut scene, &mut acc, &terminal, ROW, false);
        assert_eq!(settle(&scene, &mut acc, &frames, &mut screen), 0, "nothing composed");
        assert!(screen == full(&scene, &frames));
    }
    // The curtain's clock still composes: it is on top.
    let curtain = *scene.layers().find(|l| l.owner_pid == SHELL).expect("curtain");
    frames[SHELL as usize] += 1;
    present_as(&mut scene, &mut acc, &curtain, SCREEN, true);
    assert_eq!(settle(&scene, &mut acc, &frames, &mut screen), SCREEN.area());
    // Unlocked: the chrome no longer opaque, every pixel of it changed; the
    // Terminal's last frame shows.
    frames[SHELL as usize] += 1;
    present_as(&mut scene, &mut acc, &curtain, SCREEN, false);
    settle(&scene, &mut acc, &frames, &mut screen);
    assert!(screen == full(&scene, &frames), "the Terminal as it is now");
    frames[TERMINAL as usize] += 1;
    commit(&mut scene, &mut acc, rect_of(&terminal), TERMINAL);
    assert_eq!(settle(&scene, &mut acc, &frames, &mut screen), rect_of(&terminal).area());
}

#[test]
fn a_submit_says_nothing_of_the_pixels_so_it_covers_nothing() {
    let (mut scene, mut acc, mut frames, mut screen) = locked();
    // The shell submits its next backing whole (a refused present): no
    // longer known opaque, so what is under it composes again.
    let mut whole = layer(SHELL, 0, 0, W, H, TRUSTED_Z);
    whole.surface_handle = 77;
    for r in submit_layer(&mut scene, whole).expect("room").into_iter().flatten() {
        acc.accumulate(r);
    }
    assert!(scene.layers().all(|l| !l.opaque), "a submit clears it");
    settle(&scene, &mut acc, &frames, &mut screen);
    frames[TERMINAL as usize] += 1;
    let terminal = *scene.layers().find(|l| l.owner_pid == TERMINAL).expect("terminal");
    commit(&mut scene, &mut acc, rect_of(&terminal), TERMINAL);
    assert_eq!(settle(&scene, &mut acc, &frames, &mut screen), rect_of(&terminal).area());
}

#[test]
fn only_damage_wholly_under_the_cover_is_left() {
    let mut scene = SceneTable::new();
    let mut acc = DamageAccumulator::new();
    let under = layer(TERMINAL, 0, 0, 40, 30, Z);
    submit_layer(&mut scene, under).expect("room");
    let cover = layer(SHELL, 10, 10, 20, 10, Z);
    submit_layer(&mut scene, cover).expect("room");
    present_as(&mut scene, &mut acc, &cover, SCREEN, true);
    let parts = |r: Rect| -> Vec<(u32, u32, u32, u32)> {
        shown_by_owner(&scene, r, TERMINAL).map(|p| (p.x, p.y, p.width, p.height)).collect()
    };
    assert_eq!(parts(Rect { x: 12, y: 12, width: 5, height: 5 }), vec![], "inside: left");
    assert_eq!(
        parts(Rect { x: 8, y: 12, width: 5, height: 5 }),
        vec![(8, 12, 5, 5)],
        "across its edge: kept whole"
    );
    assert_eq!(
        parts(Rect { x: 10, y: 10, width: 20, height: 10 }),
        vec![],
        "its very rectangle: left"
    );
    // A layer over the cover is never hidden by it.
    assert_eq!(
        shown_by_owner(&scene, Rect { x: 12, y: 12, width: 5, height: 5 }, SHELL).count(),
        1
    );
    // A cover whose surface missed a paint is drawn by nothing.
    let (layers, n) = scene.z_sorted_snapshot();
    let mut missed = layers[..n].iter().find(|l| l.owner_pid == SHELL).copied().expect("cover");
    missed.miss_count = 1;
    let on = layers[..n].iter().find(|l| l.owner_pid == TERMINAL).expect("under");
    assert!(!crate::hidden::hidden(
        [missed].iter(),
        on,
        Rect { x: 12, y: 12, width: 5, height: 5 }
    ));
}

const CHROME_Z: u32 = 3_000_000;

/// The shell's chrome at the screen's size in `z`, showing `handle`.
fn chrome(handle: u64, z: u32) -> Layer {
    Layer { surface_handle: handle, ..layer(SHELL, 0, 0, W, H, z) }
}

/// The shell's paint and band move as its repaint and trusted_band::sync do:
/// the frame presented where the chrome is, then a twin placed in `to` and
/// the old band dropped. The pixels composed for each request.
fn present_then_move(
    s: (&mut SceneTable, &mut DamageAccumulator, &mut [u32; 3], &mut Vec<u64>),
    (from, to): (u32, u32),
    opaque: bool,
) -> [u64; 3] {
    let (scene, acc, frames, screen) = s;
    let on = *scene.layers().find(|l| l.owner_pid == SHELL && l.z == from).expect("the chrome");
    frames[SHELL as usize] += 1;
    present_as(scene, acc, &on, SCREEN, opaque);
    let a = settle(scene, acc, frames, screen);
    submit_as(scene, acc, chrome(on.surface_handle, to));
    let b = settle(scene, acc, frames, screen);
    drop_band_as(scene, acc, SHELL, from);
    let c = settle(scene, acc, frames, screen);
    assert!(*screen == full(scene, frames));
    [a, b, c]
}

/// The shell's unlock: the curtain moved out of the trusted band first, then
/// the desktop's chrome presented where it is now.
fn move_then_present(
    s: (&mut SceneTable, &mut DamageAccumulator, &mut [u32; 3], &mut Vec<u64>),
    (from, to): (u32, u32),
) -> [u64; 3] {
    let (scene, acc, frames, screen) = s;
    let on = *scene.layers().find(|l| l.owner_pid == SHELL && l.z == from).expect("the curtain");
    submit_as(scene, acc, chrome(on.surface_handle, to));
    let a = settle(scene, acc, frames, screen);
    drop_band_as(scene, acc, SHELL, from);
    let b = settle(scene, acc, frames, screen);
    let now = *scene.layers().find(|l| l.owner_pid == SHELL).expect("the chrome");
    frames[SHELL as usize] += 1;
    present_as(scene, acc, &now, SCREEN, false);
    let c = settle(scene, acc, frames, screen);
    assert!(*screen == full(scene, frames));
    [a, b, c]
}

#[test]
fn the_curtain_goes_up_and_comes_down_composing_the_screen_once_each() {
    let (mut scene, mut acc, mut frames) = (SceneTable::new(), DamageAccumulator::new(), [0u32; 3]);
    submit_layer(&mut scene, layer(TERMINAL, 2, 2, 40, 30, Z)).expect("room");
    submit_layer(&mut scene, chrome(70, CHROME_Z)).expect("room");
    let mut screen = full(&scene, &frames);
    let whole = SCREEN.area();
    let s = (&mut scene, &mut acc, &mut frames, &mut screen);
    assert_eq!(
        present_then_move(s, (CHROME_Z, TRUSTED_Z), true),
        [whole, 0, 0],
        "locked: the curtain once"
    );
    let terminal = *scene.layers().find(|l| l.owner_pid == TERMINAL).expect("terminal");
    frames[TERMINAL as usize] += 1;
    commit(&mut scene, &mut acc, rect_of(&terminal), TERMINAL);
    assert_eq!(settle(&scene, &mut acc, &frames, &mut screen), 0, "nothing under it");
    let s = (&mut scene, &mut acc, &mut frames, &mut screen);
    assert_eq!(
        move_then_present(s, (TRUSTED_Z, CHROME_Z)),
        [0, 0, whole],
        "unlocked: the chrome once"
    );
    // A layer between the bands (a window's popup over the chrome's band)
    // is covered and uncovered in its own place only.
    let popup = layer(TERMINAL + 10, 5, 5, 8, 6, CHROME_Z + 1);
    let mut frames4 = [0u32; 13];
    frames4[..3].copy_from_slice(&frames);
    submit_layer(&mut scene, popup).expect("room");
    let (mut acc, mut screen) = (DamageAccumulator::new(), full(&scene, &frames4));
    let on = *scene.layers().find(|l| l.owner_pid == SHELL).expect("the chrome");
    frames4[SHELL as usize] += 1;
    present_as(&mut scene, &mut acc, &on, SCREEN, true);
    assert_eq!(settle(&scene, &mut acc, &frames4, &mut screen), whole);
    submit_as(&mut scene, &mut acc, chrome(on.surface_handle, TRUSTED_Z));
    assert_eq!(
        settle(&scene, &mut acc, &frames4, &mut screen),
        rect_of(&popup).area(),
        "the popup's place"
    );
    drop_band_as(&mut scene, &mut acc, SHELL, CHROME_Z);
    assert_eq!(settle(&scene, &mut acc, &frames4, &mut screen), 0);
    assert!(screen == full(&scene, &frames4));
    submit_as(&mut scene, &mut acc, chrome(on.surface_handle, CHROME_Z));
    assert_eq!(settle(&scene, &mut acc, &frames4, &mut screen), 0, "placed under it");
    drop_band_as(&mut scene, &mut acc, SHELL, TRUSTED_Z);
    assert_eq!(
        settle(&scene, &mut acc, &frames4, &mut screen),
        rect_of(&popup).area(),
        "the popup shows again"
    );
    assert!(screen == full(&scene, &frames4));
}

#[test]
fn a_translucent_chrome_moved_between_bands_still_composes_its_place() {
    // Not opaque, a twin over or under it changes what shows through: the
    // old repaint stands.
    let (mut scene, mut acc, mut frames) = (SceneTable::new(), DamageAccumulator::new(), [0u32; 3]);
    submit_layer(&mut scene, layer(TERMINAL, 2, 2, 40, 30, Z)).expect("room");
    submit_layer(&mut scene, chrome(70, CHROME_Z)).expect("room");
    let mut screen = full(&scene, &frames);
    let whole = SCREEN.area();
    let s = (&mut scene, &mut acc, &mut frames, &mut screen);
    assert_eq!(present_then_move(s, (CHROME_Z, TRUSTED_Z), false), [whole, whole, whole]);
}

#[test]
fn a_twin_drawing_other_content_is_no_twin() {
    let mut scene = SceneTable::new();
    submit_layer(&mut scene, chrome(70, CHROME_Z)).expect("room");
    assert!(scene.set_opaque(SHELL, 70, true));
    let up = chrome(70, TRUSTED_Z);
    let on = *scene.layers().next().expect("the chrome");
    assert!(twin::opaque_twin(&scene, &up).is_some(), "the same surface, place and content");
    let drawn = crate::scene::Content { handle: 71, ..on.content };
    scene.set_content(SHELL, 70, drawn).expect("its layer");
    assert!(twin::opaque_twin(&scene, &up).is_none(), "another picture over it");
}
