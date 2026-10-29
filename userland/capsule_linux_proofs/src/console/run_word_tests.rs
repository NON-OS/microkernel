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

//! The word a served launch gives its run reaches the program only as the
//! kernel spelled it, `run\0<name>\0word\0<word>`, and only within the
//! kernel's bounds again: a window or a terminal run carries none.

use super::run_mode::{parse, Mode};
use super::run_word::{word, MOST};

const ASKED: &[u8] = b"run\0nonos-simplex-core\0word\0--user-display-name=Ada";

#[test]
fn a_served_run_carries_its_word_and_still_runs_in_its_window() {
    assert_eq!(word(ASKED), Some(&b"--user-display-name=Ada"[..]));
    assert_eq!(parse(ASKED).map(|(_, m)| m), Some(Mode::Window), "a word is no terminal");
    assert_eq!(word(&[ASKED, b"\0"].concat()), Some(&b"--user-display-name=Ada"[..]));
}

#[test]
fn no_word_without_the_kernel_s_spelling() {
    for args in [&b"run\0foot"[..], b"run\0foot\0cli", b"run\0foot\0word", b"run\0foot\0word\0"] {
        assert_eq!(word(args), None, "{args:?}");
    }
    assert_eq!(word(b"run\0foot\0cli\0--user-display-name=Ada"), None);
    assert_eq!(word(b"install\0foot\0word\0x"), None);
}

#[test]
fn a_word_past_the_kernel_s_bounds_is_dropped() {
    let at = |w: &[u8]| [&b"run\0core\0word\0"[..], w].concat();
    assert_eq!(word(&at(&[b'a'; MOST])).map(<[u8]>::len), Some(MOST));
    assert_eq!(word(&at(&[b'a'; MOST + 1])), None);
    for bad in [&b"A\x1bB"[..], b"A\x7f", b"\xff"] {
        assert_eq!(word(&at(bad)), None, "{bad:?}");
    }
}
