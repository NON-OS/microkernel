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

//! The kernel's other mid-session stops leave through the panic path, read
//! from the source, so none halts with RAM unwiped. Stops from before a
//! session exists are not among them.

macro_rules! src {
    ($p:literal) => {
        include_str!(concat!("../../../../src/", $p))
    };
}

const SITES: &[(&str, &str, &str)] = &[
    ("oom", src!("entry/oom.rs"), "[OOM] to the panic path: wipe, power off"),
    ("shootdown", src!("memory/paging/manager/shootdown/slow.rs"), "report_stuck();"),
    (
        "canary",
        src!("security/hardening/memory_sanitization/canary.rs"),
        "crate::arch::disable_interrupts();",
    ),
    ("pqclean", src!("crypto/pqclean_support/mod.rs"), "[FATAL] PQClean fatal error"),
    ("user frame", src!("process/userspace/asm.rs"), "[FATAL] Invalid user frame"),
    ("restore", src!("process/context/full/restore/restore.rs"), "println(e.as_bytes());"),
];

#[test]
fn each_mid_session_stop_goes_to_the_panic_path_next() {
    for (name, text, before) in SITES {
        let at = text.find(before).unwrap_or_else(|| panic!("{name}: missing {before}"));
        let next = text[at..]
            .lines()
            .skip(1)
            .map(str::trim)
            .find(|l| !l.is_empty() && !l.starts_with("//"))
            .unwrap_or("");
        assert_eq!(next, "crate::boot::panic::fatal()", "{name}");
    }
}

#[test]
fn the_oom_and_shootdown_stops_neither_halt_nor_write_vga_text() {
    for (name, text, _) in &SITES[..2] {
        let code: String = text.lines().filter(|l| !l.trim_start().starts_with("//")).collect();
        for banned in ["halt_loop", "0xb8000", "0xB8000"] {
            assert!(!code.contains(banned), "{name}: {banned}");
        }
    }
}
