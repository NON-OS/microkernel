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

//! The kernel's FIFO (TIS) protocol, included by `#[path]`, driven against a
//! modelled register file. Only the machine adapter (`mmio_bus`) stays out:
//! it is the one file that touches real device memory and the kernel clock.

#[path = "../../../../../../src/security/tpm/fifo/bus.rs"]
pub mod bus;
#[path = "../../../../../../src/security/tpm/fifo/fail.rs"]
pub mod fail;
#[path = "../../../../../../src/security/tpm/fifo/locality.rs"]
pub mod locality;
#[path = "../../../../../../src/security/tpm/fifo/recv.rs"]
pub mod recv;
#[path = "../../../../../../src/security/tpm/fifo/regs.rs"]
pub mod regs;
#[path = "../../../../../../src/security/tpm/fifo/send.rs"]
pub mod send;
#[path = "../../../../../../src/security/tpm/fifo/session.rs"]
pub mod session;

#[cfg(test)]
mod harness;
#[cfg(test)]
mod model;
#[cfg(test)]
mod model_bus;
#[cfg(test)]
mod model_io;
#[cfg(test)]
mod model_sts;
#[cfg(test)]
mod recv_fail_tests;
#[cfg(test)]
mod send_fail_tests;
#[cfg(test)]
mod tests;
