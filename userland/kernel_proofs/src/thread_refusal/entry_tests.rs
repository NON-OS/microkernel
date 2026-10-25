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

//! A thread killed from another CPU while a switch enters it stays dead
//! (L77c): every entry path stores Running through the guarded store, and
//! asks it before it loads the thread's stack or tables; the paths that
//! entered nothing put it back Ready the same way.

pub(super) const ENTERED: &str = include_str!("../../../../src/process/core/entered.rs");
pub(super) const SW: [(&str, &str, &str); 7] = [
    ("x86_64 resume", include_str!("../../../../src/arch/x86_64/context/switch/resume.rs"), K),
    ("x86_64 first", include_str!("../../../../src/arch/x86_64/context/switch/first_entry.rs"), K),
    (
        "x86_64 kernel",
        include_str!("../../../../src/arch/x86_64/context/switch/kernel_thread.rs"),
        K,
    ),
    ("aarch64 resume", include_str!("../../../../src/arch/aarch64/context/switch/resume.rs"), A),
    (
        "aarch64 first",
        include_str!("../../../../src/arch/aarch64/context/switch/first_entry.rs"),
        A,
    ),
    ("riscv64 resume", include_str!("../../../../src/arch/riscv64/context/switch/resume.rs"), A),
    (
        "riscv64 first",
        include_str!("../../../../src/arch/riscv64/context/switch/first_entry.rs"),
        A,
    ),
];
const PUT_BACK: [&str; 2] = [
    include_str!("../../../../src/arch/aarch64/context/switch/dispatch.rs"),
    include_str!("../../../../src/arch/riscv64/context/switch/dispatch.rs"),
];
// The first thing each arch loads of the thread it enters.
const K: &str = "gdt::set_kernel_stack(cpu, kstack)";
const A: &str = "swap_address_space(pcb)";

fn at(text: &str, needle: &str, what: &str) -> usize {
    text.find(needle).unwrap_or_else(|| panic!("{what}: missing `{needle}`"))
}

#[test]
fn no_entry_path_stores_running_or_ready_over_a_kill() {
    for (what, text, load) in SW {
        assert!(!text.contains("= ProcessState::Running"), "{what}: a plain Running store");
        assert!(!text.contains("= ProcessState::Ready"), "{what}: a plain Ready store");
        assert!(!text.contains("state.lock()"), "{what}: a state store not through entered.rs");
        let asked = at(text, "if !pcb.mark_running() {", what);
        assert!(asked < at(text, load, what), "{what}: asked after it loads the thread");
        assert!(text[asked..].contains("mark_running() {\n        return"), "{what}: enters on");
    }
    for text in PUT_BACK {
        assert!(!text.contains("= ProcessState::Ready") && text.contains("pcb.mark_ready();"));
    }
    let live = &ENTERED[at(ENTERED, "fn live_to(", "entered")..];
    let lock = at(live, "self.state.lock();", "entered");
    let dead = at(live, "ProcessState::Zombie(_) | ProcessState::Terminated(_)", "entered");
    assert!(lock < dead && dead < at(live, "if !dead {\n            *state = to;", "entered"));
}
