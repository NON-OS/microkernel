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

//! Why the host cannot serve a connection now, before any key exists: what
//! the client is told in plaintext, and the serial line where the operator
//! reads the whole reason. No TPM is `UNATTESTED`; records the machine does
//! not hold, or holds in a form that does not open, are `UNENROLLED`; a
//! fault that may pass (nsh.attest busy, the store slow) is `BUSY`. Pure,
//! so nsh_front_proofs holds it.

use alloc::format;
use alloc::vec::Vec;
use core::fmt::Debug;

use nsh_wire::Reason;

/// Why there is no server for this connection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unready<E> {
    /// The machine has no TPM: no host keys and no attestation.
    NoTpm,
    /// The host keys or the records did not come from nsh.attest now.
    NoAnswer,
    /// The machine does not hold these records yet.
    Missing { enrol: bool, table: bool, anchor: bool, list: bool },
    /// The enrolment certificate does not parse.
    Enrol,
    /// The role table or the revocation list does not open: why.
    Table(E),
    /// The role table's serial, or the list's issue time, is older than one
    /// this boot already served under.
    Older,
}

impl<E: Debug> Unready<E> {
    /// The plaintext reason the client is told.
    pub fn reason(&self) -> Reason {
        match self {
            Unready::NoTpm => Reason::Unattested,
            Unready::NoAnswer => Reason::Busy,
            Unready::Missing { .. } | Unready::Enrol | Unready::Table(_) | Unready::Older => {
                Reason::Unenrolled
            }
        }
    }

    /// The serial line, newline ended.
    pub fn line(&self) -> Vec<u8> {
        let what = match self {
            Unready::NoTpm => "no TPM, so no host keys and no attestation".into(),
            Unready::NoAnswer => "nsh.attest gave no host keys or records".into(),
            Unready::Missing { enrol, table, anchor, list } => {
                let names = [(*enrol, "enrol"), (*table, "roles"), (*anchor, "roles.anchor"), (*list, "revocations")];
                let mut s = alloc::string::String::from("not enrolled, /nonos/nsh holds no");
                for (_, name) in names.iter().filter(|(gone, _)| *gone) {
                    s.push(' ');
                    s.push_str(name);
                }
                s
            }
            Unready::Enrol => "not enrolled, /nonos/nsh/enrol is not an nsh-enrol/1 certificate".into(),
            Unready::Table(e) => format!("not enrolled, the role table or revocation list does not open: {e:?}"),
            Unready::Older => "not enrolled, the role table or revocation list is older than one already served under".into(),
        };
        format!("[NSH-FRONT] turned away: {what}\n").into_bytes()
    }
}
