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

//! Install holds setup's answers in memory for the installer and keeps none
//! on the disk this boot came from, so a stick runs setup again after an
//! install. Only USB live, which keeps state on that disk, saves there, and
//! the Mode step does not let it be chosen. The sources are held to it.

use std::path::Path;

use super::keep_choice::{answers_for, Answers, INSTALL, USB_LIVE};

const SRC: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../capsule_setup_wizard/src");

/* A file of setup's, by its path under SRC, comments cut and whitespace dropped. */
fn code(path: impl AsRef<Path>) -> String {
    let text = std::fs::read_to_string(Path::new(SRC).join(path)).expect("readable source");
    let lines: String = text.lines().map(|l| l.split("//").next().unwrap_or("")).collect();
    lines.split_whitespace().collect()
}

#[test]
fn install_and_amnesic_hold_the_answers_for_the_installer() {
    assert_eq!(answers_for(INSTALL), Answers::Stage, "the stick keeps no answers");
    assert_eq!(answers_for(0), Answers::Stage, "amnesic keeps nothing");
    assert_eq!(answers_for(USB_LIVE), Answers::Save);
    for mode in (0..=u8::MAX).filter(|&m| m != USB_LIVE) {
        assert_eq!(answers_for(mode), Answers::Stage, "row {mode}");
    }
}

#[test]
fn the_rows_are_the_mode_steps_and_usb_live_cannot_be_chosen() {
    let m = code("render/screens/mode.rs");
    assert!(m.contains("pubusecrate::keep::{INSTALL,USB_LIVE};"), "one definition");
    let rows = "[b\"Amnesic(default)\",b\"USBlive(unavailable)\",b\"Installtothiscomputer\"]";
    assert!(m.contains(rows));
    let enter = "ifctx.mode_sel==USB_LIVE&&(code==K_ENTER||code==K_ENTER_LF){returnOutcome::Stay;}";
    assert!(m.contains(enter), "ENTER does not take USB live");
}

#[test]
fn the_commit_saves_or_stages_by_the_rule_alone() {
    let rule = "matchcrate::keep::answers_for(ctx.mode_sel){Answers::Save=>crate::keep::save(ctx),Answers::Stage=>crate::keep::stage(ctx),}";
    assert!(code("render/screens/commit.rs").contains(rule));
    assert_eq!(saves_in(Path::new(SRC)), 1, "no other way to save");
    // Staged, the vfs holds the whole record where the installer's carry reads it.
    let staged = "vfs::write_file(pid,ANSWERS_PATH,&record.encode())";
    assert!(code("keep/save.rs").contains(staged));
}

/* Calls of keep::save in the sources under `dir`. */
fn saves_in(dir: &Path) -> usize {
    let mut n = 0;
    for path in std::fs::read_dir(dir).expect("readable").map(|e| e.expect("entry").path()) {
        n += match path.is_dir() {
            true => saves_in(&path),
            false => code(&path).matches("keep::save(").count(),
        };
    }
    n
}
