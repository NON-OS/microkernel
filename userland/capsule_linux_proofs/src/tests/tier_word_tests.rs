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
//! The kernel's tier allowlist, which every `qwen` request from a terminal
//! is checked against, a run on the terminal or a window: each word it
//! takes names a tier this personality ships, so a request the kernel
//! admits never reaches `run` as a name nothing answers to.

use crate::pinned::apps::{all as apps, app};

/*
 * Compiled from the kernel's own file, not a copy of its table.
 */
#[path = "../../../../src/userspace/capsule_linux/terminal/tier.rs"]
mod kernel_tier;

use kernel_tier::{argv, package, parse};

#[test]
fn every_word_the_kernel_takes_is_a_shipped_tier() {
    let mut i = 0u8;
    while let Some(name) = package(i) {
        assert!(app(&name).is_some(), "{name} is not shipped");
        assert_eq!(parse(name["qwen-".len()..].as_bytes()), Some(i), "{name}");
        assert_eq!(argv(i).unwrap(), ["run", name.as_str(), "cli"]);
        i += 1;
    }
    assert_eq!(usize::from(i), apps().count());
    assert_eq!(parse(b""), Some(0));
    assert_eq!(parse(b"small\0\0"), Some(0));
    assert_eq!(parse(b"window"), None);
}
