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

//! The panic handler and `end`, read from the source: the path is taken
//! before anything is printed, no VGA text is written, the halt line comes
//! before the wipe, and nothing on the way out takes COM1's lock.

const HANDLER: &str = include_str!("../../../../src/boot/panic/handler.rs");
const END: &str = include_str!("../../../../src/boot/panic/end.rs");

/// The handler's code, its comments left out: they name what it no longer does.
fn code() -> String {
    HANDLER.lines().filter(|l| !l.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n")
}

fn at(text: &str, needle: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("missing: {needle}"))
}

#[test]
fn the_panic_handler_takes_the_path_before_it_prints() {
    let body = &HANDLER[at(HANDLER, "fn panic(info: &PanicInfo) -> ! {")..];
    let taken = at(body, "super::end::end(|| {");
    assert!(taken < at(body, "!!! KERNEL PANIC !!!"));
    // The stop, the halt line and the wipe are `end`'s alone.
    for own in ["send_panic_ipi", "others_halted", "panic_wipe_and_off"] {
        assert!(!code().contains(own), "{own}");
    }
}

#[test]
fn the_panic_handler_writes_no_vga_text() {
    for banned in ["vga::", "show_panic", "0xB8000", "VGA_BUFFER"] {
        assert!(!code().contains(banned), "{banned}");
    }
}

#[test]
fn end_says_the_cpus_halted_then_marks_the_wipe_then_wipes() {
    let body = &END[at(END, "fn wipe_and_off(me: u64) -> ! {")..];
    let stopped = at(body, "crate::smp::send_panic_ipi();");
    let halted = at(body, "if crate::smp::others_halted() {");
    let said = at(body, "[ZEROSTATE] panic: cpus halted, wiping, powering off");
    let marked = at(body, "STAGE.store(word(WIPING, me), Ordering::Release);");
    let wiped = at(body, "crate::security::zerostate::panic_wipe_and_off()");
    assert!(stopped < halted && halted < said && said < marked && marked < wiped);
}

#[test]
fn end_claims_with_one_swap_and_takes_no_lock() {
    assert!(END.contains("STAGE.compare_exchange(IDLE, word(STOPPING, me), Ordering::AcqRel,"));
    assert!(END.contains("let found = claim.err().unwrap_or(IDLE);"));
    assert!(END.contains("Next::Park => crate::arch::halt_loop(),"));
    assert!(END.contains("crate::sys::serial::core::write_byte(b);"));
    for banned in [".lock()", "serial::print", "serial::println", "log::"] {
        assert!(!END.contains(banned), "{banned}");
    }
}
