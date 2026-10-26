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

//! What a register window's identity and access registers say is behind it,
//! pure, so the host proves it (kernel_proofs::tpm2::ident_tests).
//!
//! TPM_INTERFACE_ID_0's low nibble names the register file. An empty window
//! answers too: on most buses every read is all ones, on QEMU's q35 every
//! read is zero, and zero is the FIFO type. So neither id is believed alone:
//! a FIFO part (TIS 1.3 included, which has no identity register) is one
//! only when TPM_ACCESS_0 reads with tpmRegValidSts set and is not all ones.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Interface {
    Fifo,
    Crb,
}

const TYPE_FIFO: u32 = 0x0;
const TYPE_CRB: u32 = 0x1;
/// A TIS 1.3 part has no identity register and reads back all ones.
const TYPE_TIS13: u32 = 0xF;
/// TPM_ACCESS_0's tpmRegValidSts, which a present FIFO part sets.
const ACCESS_VALID: u8 = 0x80;

/// The interface behind the window, from TPM_INTERFACE_ID_0 (`id`) and
/// TPM_ACCESS_0 (`access`), or None when nothing answers as a TPM.
pub(crate) const fn classify(id: u32, access: u8) -> Option<Interface> {
    let fifo_answers = access != u8::MAX && access & ACCESS_VALID != 0;
    match id & 0xF {
        TYPE_CRB => Some(Interface::Crb),
        TYPE_FIFO | TYPE_TIS13 if fifo_answers => Some(Interface::Fifo),
        _ => None,
    }
}
