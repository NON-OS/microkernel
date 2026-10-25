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

//! The committed policies.txt: the reference app's three builds, the paths
//! chat_pick runs, get the box the plan gives them.

use super::line_tests::line::parse;
use crate::mask::{MODELS_RO, NET_ANON, TTY, WAYLAND};

const COMMITTED: &str = include_str!("../../../capsule_linux/policies.txt");
const CHAT_PICK: &str = include_str!("../../../capsule_linux/src/linux/install/chat_pick.rs");

#[test]
fn the_committed_file_gives_each_qwen_build_the_reference_box() {
    let lines: Vec<_> = COMMITTED.lines().filter_map(|l| parse(l).unwrap()).collect();
    let paths: Vec<_> = lines.iter().map(|l| l.store_path).collect();
    assert_eq!(paths, ["/bin/qwenchat", "/bin/qwenchat-x86_64_v2", "/bin/qwenchat-x86_64"]);
    for l in &lines {
        assert_eq!(l.mask, WAYLAND | MODELS_RO | TTY, "{}", l.store_path);
        assert!(l.mask & NET_ANON == 0, "/models is never served beside a network");
        let quoted = format!("b\"{}\"", l.store_path);
        assert!(CHAT_PICK.contains(&quoted), "chat_pick does not run {}", l.store_path);
    }
    assert!(lines.windows(2).all(|w| w[0].calls == w[1].calls), "one set for all three");
}
