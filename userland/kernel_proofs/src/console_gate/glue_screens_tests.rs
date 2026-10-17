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

//! The stop and panic screens draw past the gate, as a halted machine has
//! to say why: each is shown only on a path that ends in the halt loop,
//! with no return between the screen and the halt.

use super::text_items::{at, flat, flat_body, naming};

#[test]
fn the_screens_are_shown_only_by_a_machine_that_halts() {
    let callers = [
        ("boot/panic/handler.rs", "fn panic(", "panic_screen("),
        ("boot/stop.rs", "pub fn stop(", "show_notice("),
        ("kernel_core/init/entry/install_refusal.rs", "fn refuse_install", "show_notice("),
        ("kernel_core/init/entry/loader_refusal.rs", "fn refuse_unchecked_loader(", "show_notice("),
    ];
    let shown: Vec<String> =
        ["panic_screen(", "show_notice("].into_iter().flat_map(naming).collect();
    let mut want: Vec<&str> = callers.iter().map(|c| c.0).collect();
    want.push("sys/boot_log/notice_screen.rs");
    assert_eq!(shown.iter().filter(|f| !want.contains(&f.as_str())).count(), 0, "{shown:?}");
    for (rel, head, show) in callers {
        let b = flat_body(rel, head);
        let after = &b[at(&b, &flat(show))..];
        assert!(after.ends_with("crate::arch::halt_loop()"), "{rel} goes on past its halt");
        assert!(!after.contains("return") && !after.contains('?'), "{rel} returns past it");
        assert_eq!(after.matches("halt_loop").count(), 1, "{rel}");
    }
}
