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

//! The kept stack around an IST-like `sp`: a double fault runs on a 16 KiB
//! IST stack, not the thread's, and wherever in it `sp` stands, and wherever
//! the stack itself sits, heap or .bss, the wipe never reaches a byte of it.

use super::helpers::{covered, exact, run, span};
use super::panic_stack::stack_keep;

const GDT: &str = include_str!("../../../../src/arch/x86_64/gdt/constants.rs");
const LAYOUT: &str = include_str!("../../../../src/memory/layout/constants/percpu.rs");

/// The value of `pub const NAME: usize = <a> * <b>` or `= <n>`.
fn constant(text: &str, name: &str) -> u64 {
    let at = text.find(&format!("pub const {name}: usize = ")).unwrap_or_else(|| panic!("{name}"));
    let value = text[at..].split('=').nth(1).unwrap().split(';').next().unwrap();
    value.split('*').map(|f| f.trim().replace('_', "").parse::<u64>().unwrap()).product()
}

#[test]
fn an_ist_stack_is_no_larger_than_the_kept_window() {
    let ist = constant(GDT, "DEFAULT_STACK_SIZE");
    let kstack = constant(LAYOUT, "KSTACK_SIZE");
    assert_eq!((ist, kstack), (16 * 1024, 64 * 1024), "the sizes this proof places");
    assert!(ist <= kstack);
}

#[test]
fn no_ist_byte_is_wiped_wherever_the_stack_and_sp_stand() {
    let (ist, kstack) = (constant(GDT, "DEFAULT_STACK_SIZE"), constant(LAYOUT, "KSTACK_SIZE"));
    let heap = span(0x10_0000, 0x18_0000);
    // Inside the heap at its start, middle and end, and in .bss past it.
    for base in [0x10_0000, 0x13_3000, 0x18_0000 - ist, 0x20_0000] {
        for sp in (base..=base + ist).step_by(512) {
            let keep = [stack_keep(sp, kstack)];
            let spans = run(heap, &keep);
            for at in (base..base + ist).step_by(64) {
                assert!(!covered(&spans, at), "ist {base:#x} sp {sp:#x} byte {at:#x}");
            }
        }
    }
    exact(heap, &[stack_keep(0x13_3000 + ist / 2, kstack)]);
}

#[test]
fn the_window_clamps_at_both_ends_of_the_address_space() {
    assert_eq!(stack_keep(0x100, 0x1000), span(0, 0x1100));
    assert_eq!(stack_keep(u64::MAX - 0x100, 0x1000), span(u64::MAX - 0x1100, u64::MAX));
}
