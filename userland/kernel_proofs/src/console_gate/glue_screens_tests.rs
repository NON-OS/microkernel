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
//! to say why: each stop screen is shown only on a path that ends in the
//! halt loop, and the panic screen only on the panic's one way out, which
//! wipes and powers off; no return between the screen and either end.

use super::text::code;
use super::text_items::{at, flat, flat_body, naming};

#[test]
fn the_screens_are_shown_only_by_a_machine_that_halts() {
    let callers = [
        ("boot/stop.rs", "pub fn stop(", "show_notice("),
        ("kernel_core/init/entry/install_refusal.rs", "fn refuse_install", "show_notice("),
        ("kernel_core/init/entry/loader_refusal.rs", "fn refuse_unchecked_loader(", "show_notice("),
    ];
    let shown: Vec<String> =
        ["panic_screen(", "show_notice("].into_iter().flat_map(naming).collect();
    let mut want: Vec<&str> = callers.iter().map(|c| c.0).collect();
    want.extend(["sys/boot_log/notice_screen.rs", "boot/panic/handler.rs"]);
    assert_eq!(shown.iter().filter(|f| !want.contains(&f.as_str())).count(), 0, "{shown:?}");
    for (rel, head, show) in callers {
        let b = flat_body(rel, head);
        let after = &b[at(&b, &flat(show))..];
        assert!(after.ends_with("crate::arch::halt_loop()"), "{rel} goes on past its halt");
        assert!(!after.contains("return") && !after.contains('?'), "{rel} returns past it");
        assert_eq!(after.matches("halt_loop").count(), 1, "{rel}");
    }
}

#[test]
fn the_panic_screen_is_painted_only_on_the_way_out() {
    let b = flat_body("boot/panic/handler.rs", "fn panic(");
    assert!(b.starts_with("super::end::end(||{"), "the panic paints outside its way out");
    let after = &b[at(&b, "panic_screen(")..];
    assert!(after.ends_with("})"), "the panic goes on past its way out");
    assert!(!after.contains("return") && !after.contains('?'), "the panic returns past it");
    let end = flat_body("boot/panic/end.rs", "pub(crate) fn end(");
    assert!(end.contains("Next::Full=>{paint();wipe_and_off(me)}"), "painted, then not wiped");
    let text = code("boot/panic/end.rs");
    assert!(text.contains("pub(crate) fn end(paint: impl FnOnce()) -> ! {"));
    assert!(text.contains("fn wipe_and_off(me: u64) -> ! {"));
}
