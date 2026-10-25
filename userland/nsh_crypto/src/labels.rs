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

//! The labelled digests the hybrid keys sign, and the attestation
//! challenge. Every signed digest is `H(label || inner)` with a label that
//! names its one purpose and ends in a zero byte, so no signature made for
//! one purpose can be presented for another.

use nonos_hash::Sha512;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label {
    HostSig,
    UserSig,
    RecordSig,
    RecordCkpt,
    Grant,
    Approve,
    Enrol,
    Policy,
    Revoke,
}

impl Label {
    pub const fn bytes(self) -> &'static [u8] {
        match self {
            Label::HostSig => b"nsh/1 host-sig\0",
            Label::UserSig => b"nsh/1 user-sig\0",
            Label::RecordSig => b"nsh/1 record-sig\0",
            Label::RecordCkpt => b"nsh/1 record-ckpt\0",
            Label::Grant => b"nsh/1 grant\0",
            Label::Approve => b"nsh/1 approve\0",
            Label::Enrol => b"nsh/1 enrol\0",
            Label::Policy => b"nsh/1 policy\0",
            Label::Revoke => b"nsh/1 revoke\0",
        }
    }
}

pub fn labelled(label: Label, inner: &[u8]) -> [u8; 64] {
    let mut h = Sha512::new();
    h.update(label.bytes());
    h.update(inner);
    h.finalize()
}
