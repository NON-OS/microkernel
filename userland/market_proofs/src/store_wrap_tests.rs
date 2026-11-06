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

//! The detail pane breaks a listing's words with the toolkit's `text::wrap`
//! into the very lines the store's own wrap gave, which this keeps as `before`.

use nonos_toolkit::widgets::text::{width, wrap};

use crate::store::ui::metrics::{BODY_PX, DETAIL_PAD, DETAIL_W, SMALL_PX};

/// Descriptions from the signed index, the shortest and the longest among
/// them, and a sentence the pane says of a model tier.
const SAID: [&str; 5] = [
    "Qwen3 14B, running on this machine with no network.",
    "Qwen2.5 1.5B Instruct, running on this machine with no network. A good balance of quality and speed.",
    "Qwen3 30B-A3B, a mixture of experts with 3B active per token, running on this machine with no network. Answers far faster on a CPU than its size suggests.",
    "Qwen2.5-Coder 32B Instruct, running on this machine with no network. The strongest code tier. Slow on a CPU.",
    "The stick tier: Install imports it from a release stick with no network when this stick carries it, and downloads it otherwise.",
];

/// The wrap the store kept until the toolkit's replaced it: words split at
/// spaces, each line measured joined by one space.
fn before(text: &str, room: u32, px: f32) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut line = String::new();
    for word in text.split(' ').filter(|w| !w.is_empty()) {
        let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
        if width(&candidate, px) > room && !line.is_empty() {
            out.push(core::mem::replace(&mut line, word.to_string()));
        } else {
            line = candidate;
        }
    }
    out.extend((!line.is_empty()).then_some(line));
    out
}

fn after(text: &str, room: u32, px: f32) -> Vec<String> {
    wrap(text, room, px).into_iter().map(|(s, e)| text[s..e].to_string()).collect()
}

#[test]
fn listing_words_wrap_to_the_lines_they_did() {
    let pane = DETAIL_W - DETAIL_PAD * 2;
    assert!(after(SAID[2], pane, SMALL_PX).len() > 2, "the pane breaks the longest");
    for said in SAID {
        for room in (60..=640).step_by(3).chain([pane]) {
            for px in [SMALL_PX, BODY_PX] {
                assert_eq!(after(said, room, px), before(said, room, px), "{said} at {room}");
            }
        }
    }
}

#[test]
fn a_newline_is_where_the_two_differ() {
    let said = "first line.\nsecond";
    let room = width("first line.", SMALL_PX);
    assert_eq!(before(said, room, SMALL_PX), ["first", "line.\nsecond"]);
    assert_eq!(after(said, room, SMALL_PX), ["first line.", "second"]);
}
