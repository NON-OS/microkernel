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

//! MkAppLaunch's one word: only the capsule SERVES pairs with the core, for
//! the core's listing alone, within the prefix WORDS gives it, and only plain
//! text within MOST bytes; no word is today's call, from anyone.

use super::app_launch_word::{admit_word, MOST};
use super::serve_rule::serves::{LISTINGS, WORDS};

const CORE: &str = "linux.kali.nonos-simplex-core";
const NAME: &[u8] = b"--user-display-name=Ada";

fn window(capsule: &str) -> bool {
    capsule == "app.simplex"
}

fn nobody(_: &str) -> bool {
    false
}

#[test]
fn the_window_names_its_core_s_profile_and_nothing_else() {
    let t = (LISTINGS, WORDS);
    assert_eq!(admit_word(window, CORE, NAME, t), Ok(Some("--user-display-name=Ada")));
    assert_eq!(admit_word(window, CORE, b"", t), Ok(None), "no word is today's call");
    assert_eq!(admit_word(nobody, CORE, b"", t), Ok(None));
    let others = [&b"-p 9999"[..], b"--chat-server-port=1", b"--user-display-name="];
    for other in others.into_iter().chain([&b"--user-display-name"[..], b" --user-display-name=A"])
    {
        assert_eq!(admit_word(window, CORE, other, t), Err(()), "{other:?}");
    }
}

#[test]
fn only_the_paired_capsule_for_its_own_listing_may_give_one() {
    let t = (LISTINGS, WORDS);
    assert_eq!(admit_word(nobody, CORE, NAME, t), Err(()));
    assert_eq!(admit_word(|c: &str| c == "app.terminal", CORE, NAME, t), Err(()));
    assert_eq!(admit_word(window, "linux.kali.nonos-simplex-chat", NAME, t), Err(()));
}

#[test]
fn a_word_is_plain_text_within_its_bound() {
    let t = (LISTINGS, WORDS);
    let long = [NAME, &[b'a'; MOST][..]].concat();
    assert!(matches!(admit_word(window, CORE, &long[..MOST], t), Ok(Some(_))));
    assert_eq!(admit_word(window, CORE, &long[..MOST + 1], t), Err(()));
    let bad = [
        &b"--user-display-name=A\nB"[..],
        b"--user-display-name=A\x7f",
        b"--user-display-name=\xff",
    ];
    for word in bad {
        assert_eq!(admit_word(window, CORE, word, t), Err(()), "{word:?}");
    }
    let named = "--user-display-name=Zo\u{eb} \u{674e}".as_bytes();
    assert!(matches!(admit_word(window, CORE, named, t), Ok(Some(_))), "any script");
}
