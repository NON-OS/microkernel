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

/// The IPC call sites, endpoints and call paths among them.
pub(super) fn ipc_sites() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
    vec![
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
        (
            "ipc/call/answer.rs",
            include_str!("../../../../src/syscall/microkernel/ipc/call/answer.rs"),
            vec![],
        ),
        (
            "ipc/call/gpu_bench.rs",
            include_str!("../../../../src/syscall/microkernel/ipc/call/gpu_bench.rs"),
            vec![],
        ),
        (
            "ipc/call/token.rs",
            include_str!("../../../../src/syscall/microkernel/ipc/call/token.rs"),
            vec![],
        ),
    ]
}
