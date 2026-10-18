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

//! No `cli` or `sti` is a `nomem` asm. Marked `nomem`, the compiler may move
//! a critical section's loads and stores out past the instruction that was
//! meant to bracket them. Every file that held one is read here.

macro_rules! src {
    ($p:literal) => {
        ($p, include_str!(concat!("../../../src/", $p)))
    };
}

const SITES: [(&str, &str); 16] = [
    src!("arch/x86_64/interrupt/irq_flag.rs"),
    src!("smp/ap/go_live.rs"),
    src!("smp/ap/entry.rs"),
    src!("smp/ap/idle.rs"),
    src!("boot/main/core_init/init_core_systems.rs"),
    src!("arch/trap/contract/backend_x86_64.rs"),
    src!("arch/cpu/idle_cpu.rs"),
    src!("arch/x86_64/cpu/control_intr.rs"),
    src!("arch/x86_64/acpi/power_reboot.rs"),
    src!("arch/x86_64/acpi/power_sleep.rs"),
    src!("arch/x86_64/abi.rs"),
    src!("arch/x86_64/boot/cpu_ops/intrinsics_interrupt.rs"),
    src!("arch/x86_64/idle.rs"),
    src!("arch/x86_64/idt/ops/interrupt.rs"),
    src!("arch/x86_64/idt/ops/init.rs"),
    src!("arch/x86_64/interrupt/mod.rs"),
];

fn touches_if(line: &str) -> bool {
    line.contains("asm!") && ["\"cli", "\"sti", "cli;", "sti;"].iter().any(|i| line.contains(i))
}

#[test]
fn no_cli_or_sti_is_nomem() {
    for (path, src) in SITES {
        for (n, line) in src.lines().enumerate() {
            assert!(!(touches_if(line) && line.contains("nomem")), "{path}:{}", n + 1);
        }
    }
}

#[test]
fn the_helpers_are_the_only_plain_cli_and_sti() {
    // A fused sequence (`"cli", "hlt"`) names its second instruction before
    // `options`; only a lone `cli` or `sti` goes straight to it.
    let lone = |s: &str, i: &str| s.matches(&format!("asm!(\"{i}\", options")).count();
    let plain = |s: &str| lone(s, "cli") + lone(s, "sti");
    for (path, src) in SITES {
        let want = if path.ends_with("irq_flag.rs") { 2 } else { 0 };
        assert_eq!(plain(src), want, "{path}");
    }
}
