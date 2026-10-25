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

//! Between a job's stages: the frames queued up, answered.

use nonos_libc::ipc::MK_IPC_NO_WAIT;
use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};
use nonos_proof_abi::wire::{Header, PROVE_MAGIC};

use super::answer::answer;
use super::state::Job;

/// While a job runs: answer what has queued up, without waiting.
pub fn drain(job: &mut Job, buf: &mut [u8]) {
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), buf.len(), MK_IPC_NO_WAIT, &mut sender);
        let Some(n) = usize::try_from(n).ok().filter(|&n| n > 0 && sender != 0) else {
            return;
        };
        if let Some((h, body)) = Header::parse(&buf[..n], PROVE_MAGIC) {
            let tx = answer(Some(job), sender, &h, body);
            let _ = mk_ipc_reply(sender, tx.as_ptr(), tx.len());
        }
    }
}
