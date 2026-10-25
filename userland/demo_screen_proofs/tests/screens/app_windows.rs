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

//! Every app window holds what its app paints. An app laid out at a fixed
//! size names that size as its content and asks for the window around it
//! (window_for); the content area the runner gives it, after the fit rule
//! and the frame, is then at least that, on every display mode. Every other
//! app lays itself out from the buffer it is handed (app_census.rs).

use nonos_app_skeleton::runner::chrome::quarters_for;
use nonos_app_skeleton::runner::fit_rule::fit_manifest;
use nonos_app_skeleton::{window_for, AppManifest};
use nonos_toolkit::decorations::{content_rect, content_rect_at};

macro_rules! fixed {
    ($($m:ident = $path:literal;)*) => {
        $(#[path = $path] mod $m;)*
        pub(crate) const FIXED: &[(&str, fn() -> AppManifest, u32, u32)] =
            &[$((stringify!($m), $m::manifest, $m::WIDTH, $m::HEIGHT)),*];
    };
}

fixed! {
    clock = "../../../capsule_clock/src/clock/manifest.rs";
    egui_proof = "../../../capsule_egui_proof/src/egui_proof/manifest.rs";
    game_2048 = "../../../capsule_game_2048/src/game/manifest.rs";
    gui_demo = "../../../capsule_gui_demo/src/gui_demo/manifest.rs";
    gui_proof = "../../../capsule_gui_proof/src/manifest.rs";
    hello = "../../../capsule_hello/src/hello/manifest.rs";
    prove = "../../../capsule_prove/src/app/manifest.rs";
}

#[test]
fn a_fixed_layout_app_gets_its_whole_layout_on_every_mode() {
    for &(app, manifest, w, h) in FIXED {
        for (dw, dh) in [(1920, 1080), (2560, 1440), (1280, 800), (1024, 768)] {
            let at = fit_manifest(manifest(), dw, dh);
            let c = content_rect_at(at.width, at.height, false, quarters_for(dw, dh));
            assert!(c.w >= w && c.h >= h, "{app} on {dw}x{dh}: {}x{} for {w}x{h}", c.w, c.h);
        }
    }
}

#[test]
fn the_window_for_a_content_area_frames_exactly_that_area() {
    for (w, h) in [(360, 480), (1, 1), (760, 380), (2000, 1200)] {
        let (ww, wh) = window_for(w, h);
        let c = content_rect(ww, wh, false);
        assert_eq!((c.w, c.h), (w, h));
    }
}
