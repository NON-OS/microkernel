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

//! Syscall entry contract. Shared dispatch with a structurally
//! unbypassable capability check. Per-arch entry shims call
//! `dispatch(SyscallNumber, SyscallArgs)` after extracting the syscall
//! number and the six argument registers. The x86_64 shim lives in
//! `crate::arch::x86_64::syscall::manager::entry`. The aarch64 shim is
//! `arch::aarch64::exceptions::handlers::svc` and the riscv64 one is
//! `arch::riscv64::interrupts::handlers::syscall`; both call this same
//! `dispatch`.

mod args;
mod cap_table;
mod capability;
mod dispatch;
mod resolver;

pub use args::SyscallArgs;
pub use capability::Capability;
pub use dispatch::dispatch;
