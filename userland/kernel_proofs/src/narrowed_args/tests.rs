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

use super::narrow::{u16_arg, u32_arg, u8_arg};

const EDGES: [u64; 9] = [
    0,
    1,
    0xFFFF,
    0xFFFF_FFFF,
    0x1_0000_0000,
    0x1_0000_0001,
    0x1_0000_FFFF_0005,
    0x8000_0000_0000_0000,
    u64::MAX,
];

/// The trimmed lines of `src` that cast to a narrower integer, comments aside.
fn narrowing_casts(src: &str) -> Vec<&str> {
    const CASTS: [&str; 6] = [" as u32", " as u16", " as u8", " as i32", " as i16", " as i8"];
    src.lines()
        .map(str::trim)
        .filter(|l| !(l.starts_with("//") || l.starts_with('*') || l.starts_with("/*")))
        .filter(|l| {
            CASTS.iter().any(|c| {
                l.match_indices(c).any(|(i, _)| {
                    !l[i + c.len()..].starts_with(|ch: char| ch.is_ascii_alphanumeric())
                })
            })
        })
        .collect()
}

/// The pid, port and endpoint call sites, with the casts each may keep.
fn id_sites() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
    vec![
        (
            "dispatch/capability.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/capability.rs"),
            vec![],
        ),
        (
            "dispatch/ipc.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/ipc.rs"),
            vec![],
        ),
        (
            "dispatch/process.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/process.rs"),
            /*
             * Kept on purpose: mmap's protection and flags are bit sets whose
             * unknown bits are ignored high or low, an exit status is a C int,
             * and a futex compares one 32-bit word.
             */
            vec![
                "SYS_MMAP => sys_mmap(a.a0, a.a1 as usize, a.a2 as u32, a.a3 as u32),",
                "SYS_EXIT => sys_exit(a.a0 as i32),",
                "SYS_FUTEX_WAIT => sys_futex_wait(a.a0, a.a1 as u32, a.a2),",
            ],
        ),
        ("wait.rs", include_str!("../../../../src/syscall/microkernel/wait.rs"), vec![]),
        (
            "kill.rs",
            include_str!("../../../../src/syscall/microkernel/kill.rs"),
            // The signal is one of three small numbers by the time these run.
            vec![
                "terminate_current_with_signal(sig as u8);",
                "crate::process::exit::teardown(target, 128 + sig as i32, true);",
            ],
        ),
        ("ipc/send.rs", include_str!("../../../../src/syscall/microkernel/ipc/send.rs"), vec![]),
        (
            "ipc/send_caps.rs",
            include_str!("../../../../src/syscall/microkernel/ipc/send_caps.rs"),
            vec![],
        ),
        (
            "ipc/inbox_name.rs",
            include_str!("../../../../src/syscall/microkernel/ipc/inbox_name.rs"),
            vec![],
        ),
        (
            "ipc/call/sys_ipc_call.rs",
            include_str!("../../../../src/syscall/microkernel/ipc/call/sys_ipc_call.rs"),
            vec![],
        ),
    ]
}

/// The device call sites, with the casts each may keep.
fn device_sites() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
    vec![
        (
            "dispatch/irq.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/irq.rs"),
            vec![],
        ),
        (
            "dispatch/dma.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/dma.rs"),
            vec![],
        ),
        (
            "dispatch/pio.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/pio.rs"),
            vec![],
        ),
        (
            "dispatch/debug.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/debug.rs"),
            vec![],
        ),
        ("pci.rs", include_str!("../../../../src/syscall/microkernel/pci.rs"), vec![]),
        (
            "dispatch/device.rs",
            include_str!("../../../../src/syscall/microkernel/dispatch/device.rs"),
            vec![],
        ),
        (
            "router/surface_handlers.rs",
            include_str!("../../../../src/syscall/dispatch/router/surface_handlers.rs"),
            // A digit of a u32 being printed, not a register.
            vec!["tmp[k] = b'0' + (v % 10) as u8;"],
        ),
        (
            "router/admin/policy_push/entry.rs",
            include_str!("../../../../src/syscall/dispatch/router/admin/policy_push/entry.rs"),
            vec![],
        ),
    ]
}

#[test]
fn every_value_a_field_holds_is_kept_whole() {
    for raw in EDGES.into_iter().chain([0xFF, 0x100, 0x1_0000, 0x1_0100]) {
        match u32_arg(raw) {
            Some(v) => assert_eq!(u64::from(v), raw, "{raw:#x} came back as {v:#x}"),
            None => assert!(raw > u64::from(u32::MAX), "{raw:#x} fits and was refused"),
        }
        match u16_arg(raw) {
            Some(v) => assert_eq!(u64::from(v), raw, "{raw:#x} came back as {v:#x}"),
            None => assert!(raw > u64::from(u16::MAX), "{raw:#x} fits and was refused"),
        }
        match u8_arg(raw) {
            Some(v) => assert_eq!(u64::from(v), raw, "{raw:#x} came back as {v:#x}"),
            None => assert!(raw > u64::from(u8::MAX), "{raw:#x} fits and was refused"),
        }
    }
}

#[test]
fn a_value_wider_than_the_field_is_refused() {
    assert_eq!(u32_arg(u64::from(u32::MAX)), Some(u32::MAX));
    for raw in [0x1_0000_0000, 0x1_0000_0001, 0x1_0000_FFFF_0005, u64::MAX] {
        assert_eq!(u32_arg(raw), None, "{raw:#x} was cut down");
    }
    // A config write of 0x1_0006 is not a write of 6, nor BAR 256 BAR 0.
    assert_eq!(u16_arg(0xFFFF), Some(0xFFFF));
    assert_eq!(u16_arg(0x1_0006), None);
    assert_eq!(u8_arg(0xFF), Some(0xFF));
    assert_eq!(u8_arg(0x100), None);
}

#[test]
fn no_id_call_site_truncates_a_register() {
    for (name, src, kept) in id_sites() {
        assert_eq!(narrowing_casts(src), kept, "{name} narrows a register with a cast");
    }
}

#[test]
fn no_device_call_site_truncates_a_register() {
    for (name, src, kept) in device_sites() {
        assert_eq!(narrowing_casts(src), kept, "{name} narrows a register with a cast");
    }
}

#[test]
fn the_scan_sees_the_casts_it_was_written_for() {
    assert_eq!(narrowing_casts("    let target = pid as u32;"), ["let target = pid as u32;"]);
    assert_eq!(narrowing_casts("lookup_port(ep as u32)"), ["lookup_port(ep as u32)"]);
    assert!(narrowing_casts("// as u32 in a comment\nlet n = len as usize;").is_empty());
    assert!(narrowing_casts("let w = x as u32x4;").is_empty());
}
