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

//! Which build of the chat program a tier starts: under QEMU's software CPU
//! the plain x86-64 build, on hardware the best one the CPU runs, and any
//! other build only with a line that says why.

use alloc::vec::Vec;

use crate::models::apps::CHAT;
use crate::models::chat_pick::{pick, CHAT_PLAIN, CHAT_V2};

#[test]
fn qemu_software_cpu_starts_the_plain_build_first() {
    for v3 in [true, false] {
        let builds = pick(CHAT, true, v3);
        assert_eq!(builds[0].0, CHAT_PLAIN, "v3 {v3}");
        assert!(core::str::from_utf8(builds[0].1).unwrap().contains("QEMU software CPU"));
        // A store without it says so, and runs a build this CPU runs.
        for (build, said) in &builds[1..] {
            let said = core::str::from_utf8(said).unwrap();
            assert!(said.contains("no plain x86-64 build"), "{said}");
            assert!(*build != CHAT || v3, "a v3 build offered to a CPU without it");
        }
    }
}

#[test]
fn hardware_starts_the_best_build_it_runs_and_says_any_other() {
    let v3 = pick(CHAT, false, true);
    assert_eq!(v3.iter().map(|b| b.0).collect::<Vec<_>>(), [CHAT, CHAT_V2, CHAT_PLAIN]);
    assert!(v3[0].1.is_empty());
    let v2 = pick(CHAT, false, false);
    assert_eq!(v2.iter().map(|b| b.0).collect::<Vec<_>>(), [CHAT_V2, CHAT_PLAIN]);
    // Every build but the best one says why it was chosen.
    for (_, said) in v3.iter().skip(1).chain(v2.iter()) {
        assert!(said.starts_with(b"qwen: ") && said.ends_with(b"\n"), "{said:?}");
    }
    // A program with no other builds is only itself.
    assert_eq!(pick(b"/bin/other", true, false), [(&b"/bin/other"[..], &b""[..])]);
}
