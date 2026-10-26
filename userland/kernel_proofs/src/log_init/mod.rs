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

//! The kernel logger is started: each architecture's boot calls
//! `crate::log::init()` once, right after the heap is up, and nothing else
//! does. Without it (aa4d73213 to this fix) every kernel warn, every
//! [CAP-DENY] and [IPC-DENY], reached no sink. The log is built in place
//! behind an IrqMutex, and there is no VGA text backend under GOP.

use std::path::Path;

const X86: &str = include_str!("../../../../src/boot/main/core_init/cpu_tables.rs");
const ARM: &str = include_str!("../../../../src/arch/aarch64/boot/core_entry.rs");
const RISCV: &str = include_str!("../../../../src/arch/riscv64/boot/entry.rs");
const STATE: &str = include_str!("../../../../src/log/manager/state.rs");
const LIFECYCLE: &str = include_str!("../../../../src/log/manager/lifecycle.rs");
const CALL: &str = "crate::log::init();";

fn calls_in(dir: &Path) -> usize {
    std::fs::read_dir(dir).unwrap().flatten().map(|e| e.path()).fold(0, |n, p| {
        if p.is_dir() {
            n + calls_in(&p)
        } else if p.extension().is_some_and(|x| x == "rs") {
            let s = std::fs::read_to_string(&p).unwrap();
            n + s.matches(CALL).count() + s.matches("init_logger()").count()
        } else {
            n
        }
    })
}

#[test]
fn each_boot_starts_the_logger_once_after_the_heap() {
    for (arch, boot) in [("x86_64", X86), ("aarch64", ARM), ("riscv64", RISCV)] {
        assert_eq!(boot.matches(CALL).count(), 1, "{arch}");
        let heap = boot.find("init_bootstrap();").expect("the heap");
        assert!(heap < boot.find(CALL).unwrap(), "{arch}: after the heap");
    }
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src");
    assert_eq!(calls_in(&src), 3, "nothing else starts it");
}

#[test]
fn the_log_is_built_in_place_and_has_no_vga_text_backend() {
    let built = "IrqMutex<Option<LogManager>> = IrqMutex::new(Some(LogManager::new()));";
    assert!(STATE.contains(built));
    assert!(!LIFECYCLE.contains("VgaBackend") && !LIFECYCLE.contains("LogManager::new()"));
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/log/backend/vga.rs");
    assert!(!src.exists(), "the 0xB8000 writer is gone");
}
