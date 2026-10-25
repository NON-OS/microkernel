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

//! The selftest image's deliberate panic: never without the feature, only
//! on its own word with it, and checked right after the box is armed and
//! its program word read, so the box is armed when it panics.

use crate::boxed::selftest_panic::maybe;

#[cfg(not(feature = "selftest-panic"))]
#[test]
fn without_the_feature_even_its_word_runs() {
    for w in [None, Some(&b"--selftest-panic"[..]), Some(b"sh")] {
        maybe(w);
    }
}

#[cfg(feature = "selftest-panic")]
#[test]
#[should_panic(expected = "selftest: capsule_linux panics on purpose")]
fn with_the_feature_its_word_panics() {
    maybe(Some(crate::boxed::selftest_panic::WORD));
}

#[cfg(feature = "selftest-panic")]
#[test]
fn with_the_feature_any_other_word_runs() {
    for w in [None, Some(&b"sh"[..]), Some(b"--selftest-panic2"), Some(b"--selftest")] {
        maybe(w);
    }
}

#[test]
fn it_is_asked_after_the_box_is_armed_and_its_program_read() {
    let src = include_str!("../../../../capsule_linux/src/linux/terminal_box.rs");
    let armed = src.find("crate::linux::boxed::arm(policy, key)").unwrap();
    let read = src[armed..].find("first = parts.next()").unwrap() + armed;
    let asked = src.find("super::selftest_panic::maybe(first);").unwrap();
    assert!(armed < read && read < asked);
    assert_eq!(src.matches("selftest_panic::").count(), 1);
}
