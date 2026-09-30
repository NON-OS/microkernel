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

//! FIFO (TIS) transport, at runtime.
//!
//! Most discrete TPM 2.0 chips present this interface rather than CRB:
//! command bytes go in through a data port a burst at a time, the part is
//! told to go, and the response comes back out of the same port. QEMU's
//! `tpm-tis` does the same.
//!
//! Everything but `mmio_bus` is generic over [`bus::FifoBus`], so the host
//! proofs drive the shipping protocol against a modelled register file.

pub(crate) mod bus;
pub(crate) mod fail;
pub(crate) mod locality;
mod mmio_bus;
pub(crate) mod recv;
pub(crate) mod regs;
pub(crate) mod send;
pub(crate) mod session;

pub(in crate::security::tpm) use mmio_bus::transact;
