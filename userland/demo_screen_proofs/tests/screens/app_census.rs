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

//! Each app with a manifest is laid out at a fixed size, and proven to fit
//! in app_windows.rs, or from the buffer it is painted into; a new one is
//! placed in one list or the other before it ships.

use std::fs;
use std::path::Path;

use super::app_windows::FIXED;

/// Laid out from the buffer they are painted into (fb.width, fb.height).
const ADAPTIVE: &str = "about app_store audio_player browser calculator file_manager \
    image_viewer input_proof install mdview process_manager qrgen settings snake terminal \
    text_editor video_player wallet_nonos";

#[test]
fn every_app_with_a_manifest_is_fixed_here_or_adaptive() {
    let mut seen: Vec<String> = Vec::new();
    for entry in fs::read_dir("..").expect("userland").flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(app) = name.strip_prefix("capsule_") else { continue };
        if has_manifest(&entry.path().join("src")) {
            seen.push(app.into());
        }
    }
    seen.sort();
    let mut listed: Vec<String> = FIXED.iter().map(|f| f.0.into()).collect();
    listed.extend(ADAPTIVE.split_whitespace().map(String::from));
    listed.sort();
    assert_eq!(seen, listed, "an app is in exactly one list");
}

fn has_manifest(dir: &Path) -> bool {
    fs::read_dir(dir).into_iter().flatten().flatten().any(|e| {
        let p = e.path();
        if p.is_dir() {
            return has_manifest(&p);
        }
        fs::read_to_string(&p).is_ok_and(|s| s.contains("AppManifest {"))
    })
}

#[test]
fn adaptive_apps_paint_to_the_buffer_not_their_manifest_size() {
    let fm = |f: &str| fs::read_to_string(format!("../capsule_file_manager/src/fm/{f}")).unwrap();
    for f in ["help.rs", "layout.rs"] {
        assert!(!fm(f).contains("manifest::"), "file_manager {f} reads the window it is given");
    }
    let paint = fs::read_to_string("../capsule_browser/src/browser/app/paint.rs").unwrap();
    let flat: String = paint.split_whitespace().collect();
    assert!(flat.contains("d.width!=fb.width)"), "a text page is laid out at the window's width");
    assert!(flat.contains("doc.reflow(fb.width,"));
}
