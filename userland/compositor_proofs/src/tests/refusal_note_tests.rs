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

//! H-06(a): while no display has come up, the compositor says why once at
//! once, once more 10 s on, and nothing else; the reason is the kernel's.

use super::refusal_note::{note, reason, Note, LOG_AFTER_MS};

const WAIT: &str = include_str!("../../../compositor/src/wait_for_setup.rs");
const LINE: &str = include_str!("../../../compositor/src/waiting_line.rs");

/// What is said over `ms`, asked every 20 ms as wait_for_setup asks.
fn said_over(ms: i64) -> Vec<(i64, Note)> {
    let (mut said, mut logged, mut out) = (false, false, Vec::new());
    for t in (0..ms).step_by(20) {
        match note(t, said, logged) {
            Note::Say => said = true,
            Note::Log => logged = true,
            Note::Quiet => continue,
        }
        out.push((t, note(t, false, false)));
    }
    out
}

#[test]
fn said_at_once_then_once_more_at_the_bound_and_never_again() {
    let notes = said_over(60_000);
    assert_eq!(notes.len(), 2, "{notes:?}");
    assert_eq!(notes[0].0, 0, "said at the first failed ask");
    assert_eq!(notes[1].0, LOG_AFTER_MS, "logged at the bound, not before");
    assert_eq!(LOG_AFTER_MS, 10_000);
}

#[test]
fn nothing_is_logged_before_the_bound_and_nothing_twice() {
    assert_eq!(note(LOG_AFTER_MS - 1, true, false), Note::Quiet);
    assert_eq!(note(LOG_AFTER_MS, true, false), Note::Log);
    assert_eq!(note(LOG_AFTER_MS * 6, true, true), Note::Quiet);
    assert_eq!(note(5, true, false), Note::Quiet, "said twice");
}

#[test]
fn every_reason_the_kernel_gives_is_named_and_its_depth_ignored() {
    let named: Vec<&str> = [0u32, 1, 2, 3, 0xFD, 0xFE, 0xFF].iter().map(|&w| reason(w)).collect();
    let unique: std::collections::BTreeSet<&str> = named.iter().copied().collect();
    assert_eq!(unique.len(), named.len(), "two reasons said alike");
    assert_eq!(reason(0x1002), reason(2), "bits 8 to 15 are the caller's");
    assert!(reason(0x42).contains("does not know"));
    assert!(reason(1).contains("PixelBltOnly"));
}

#[test]
fn the_wait_names_the_kernels_reason_and_keeps_waiting() {
    assert!(LINE.contains("if nonos_display_refusal(0, &mut word as *mut u32) < 0 {"));
    assert!(LINE.contains("({} bits a pixel)\", reason(word), (word >> 8) & 0xFF)"));
    assert!(LINE.contains("String::from(gop_why)"), "with a framebuffer, run_gop's own reason");
    assert_eq!(WAIT.matches("say(&waiting_line(").count(), 2, "said only as note says");
    assert!(WAIT.contains("match note(since, said, logged) {"));
    assert!(!WAIT.contains("return Err"), "the wait gives up");
}
