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

//! TPM_PPI (18): the TPM's Physical Presence Interface, read-only, for the
//! shell to say what the firmware lets the OS ask of the TPM. The body:
//! the _DSM's supported functions u64, the version's length u8 and bytes,
//! then a count u8 and each allowed operation's number u8 and answer u8
//! (1 firmware only, 2 blocked, 3 needs a present user, 4 does not).
//! ENOENT without a TPM that has the PPI. No PPI request is ever written.

use crate::boot::{evaluator, Acpi};
use crate::server::protocol::ENOENT;
use alloc::vec::Vec;
use nonos_aml::tpm_ppi::read_ppi;

pub fn tpm_ppi(acpi: &mut Acpi) -> (i32, Vec<u8>) {
    let Acpi { aml, host, devices, .. } = acpi;
    let Some(p) = read_ppi(aml, &mut evaluator(host), devices) else { return (ENOENT, Vec::new()) };
    let mut b = p.functions.to_le_bytes().to_vec();
    let version = p.version.unwrap_or_default();
    let version = &version[..version.len().min(32)];
    b.push(version.len() as u8);
    b.extend_from_slice(version);
    b.push(p.ops.len() as u8);
    for (op, answer) in p.ops {
        b.extend_from_slice(&[op, answer]);
    }
    (0, b)
}
