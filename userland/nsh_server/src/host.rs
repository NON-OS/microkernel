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

//! What the host lends the server: its hybrid keys, the kernel's
//! attestation, and its trusted role table and revocation list.

use alloc::boxed::Box;
use alloc::vec::Vec;
use nsh_wire::keys::{ED_PUBLIC_LEN, ED_SIG_LEN, ML_PUBLIC_LEN, ML_SIG_LEN};

/// Fills the buffer with fresh random bytes, or says it could not.
pub type Rand<'a> = &'a mut dyn FnMut(&mut [u8]) -> bool;

pub struct HostSig {
    pub ed: [u8; ED_SIG_LEN],
    pub ml: Box<[u8; ML_SIG_LEN]>,
}

/// The host keys: both halves sign every digest, or nothing is signed.
pub trait HostSigner {
    fn ed_public(&self) -> &[u8; ED_PUBLIC_LEN];
    fn ml_public(&self) -> &[u8; ML_PUBLIC_LEN];
    fn sign(&self, digest: &[u8; 64], rand: Rand<'_>) -> Option<HostSig>;
}

/// The kernel's attestation fault, as its syscall returned it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttestFault {
    /// The kernel refused, with this status.
    Kernel(i64),
    /// The kernel answered with something no client could accept: a
    /// document or entry list over its cap, or entries not whole records.
    Malformed,
}

/// The kernel's `NONOSATT` version 5 document over a challenge, and the
/// attestation registry's entries.
pub trait Attestation {
    fn document(&mut self, challenge: &[u8; 32]) -> Result<Vec<u8>, AttestFault>;
    fn entries(&mut self) -> Result<Vec<u8>, AttestFault>;
}
