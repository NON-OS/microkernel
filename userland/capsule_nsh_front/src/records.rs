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

//! The machine's records for one connection, from nsh.attest: the
//! enrolment certificate, the role table, its anchor and the revocation
//! list, each as the operator put it under /nonos/nsh. Read for every
//! connection, so a list refreshed on the machine applies to the next
//! login with no restart.

use alloc::vec;
use alloc::vec::Vec;

use nsh_front_wire::{split_records, ANCHOR_LEN, OP_RECORDS};
use nsh_roles::PolicyError;

use crate::relay_said::{Fault, ST_NO_TPM};
use crate::unready::Unready;

/// The answer's largest frame, under the kernel's 1 MiB message.
const RESP_CAP: usize = 1 << 20;

/// The four records, whole.
pub struct Held {
    pub enrol: Vec<u8>,
    pub table: Vec<u8>,
    pub anchor: [u8; ANCHOR_LEN],
    pub list: Vec<u8>,
}

/// The records from nsh.attest on `port`.
pub fn fetch(port: u32) -> Result<Held, Unready<PolicyError>> {
    let mut resp = vec![0u8; RESP_CAP];
    let body = match crate::host::relay(u64::from(port), OP_RECORDS, &[], &mut resp) {
        Ok(body) => body,
        Err(Fault::Status(ST_NO_TPM)) => return Err(Unready::NoTpm),
        Err(_) => return Err(Unready::NoAnswer),
    };
    let r = split_records(&body).ok_or(Unready::NoAnswer)?;
    if !r.complete() {
        return Err(Unready::Missing {
            enrol: r.enrol.is_empty(),
            table: r.table.is_empty(),
            anchor: r.anchor.is_empty(),
            list: r.list.is_empty(),
        });
    }
    let anchor: [u8; ANCHOR_LEN] = r.anchor.try_into().map_err(|_| Unready::NoAnswer)?;
    Ok(Held { enrol: r.enrol.to_vec(), table: r.table.to_vec(), anchor, list: r.list.to_vec() })
}
