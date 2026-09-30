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

//! Picking the interface and running a command through it.
//!
//! Firmware TPMs (Intel PTT, AMD fTPM, QEMU `tpm-crb`) present CRB; most
//! discrete TPM 2.0 chips (and QEMU `tpm-tis`) present the FIFO. Callers see
//! neither: they hand [`transact`] a command and get the response.

mod acpi;
mod detect;
mod dispatch;
mod ident;

pub use dispatch::transact;
