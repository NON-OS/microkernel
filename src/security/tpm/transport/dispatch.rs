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

//! One entry point for every TPM command, whichever interface the part has.

use spin::Mutex;

use super::detect::detect;
use super::ident::Interface;
use crate::security::tpm::error::TpmError;
use crate::security::tpm::{crb, fifo};

/// What detection found, kept after the first attempt either way: a part
/// is not hot-plugged, and a missing one would otherwise be looked for and
/// logged again on every command. The lock also makes one command at a time
/// the rule. Two callers interleaving bytes in one FIFO, or one ringing the
/// CRB doorbell while the other is still writing, would each get the
/// other's response.
static PART: Mutex<Option<Result<Interface, TpmError>>> = Mutex::new(None);

/// Run one command and copy the response into `out`.
///
/// # Safety
/// The caller owns what the command means. This owns only the transport.
pub unsafe fn transact(cmd: &[u8], out: &mut [u8]) -> Result<usize, TpmError> {
    let mut part = PART.lock();
    let found = *part.get_or_insert_with(detect);
    let interface = found?;
    match interface {
        // SAFETY: eK@nonos.systems - the caller accepted the command's
        // meaning; the lock held here makes this the only transaction.
        Interface::Fifo => unsafe { fifo::transact(cmd, out) },
        // SAFETY: eK@nonos.systems - as above.
        Interface::Crb => unsafe { crb::transact(cmd, out) },
    }
}

