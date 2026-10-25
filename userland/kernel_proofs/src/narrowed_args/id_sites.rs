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

/// The pid, port and endpoint call sites, with the casts each may keep.
pub(super) fn id_sites() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
    let mut sites = vec![
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
    ];
    sites.extend(super::ipc_sites::ipc_sites());
    sites
}
