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

//! One ask on the fetcher's own endpoint, waiting up to a given time.

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup};

use crate::status_wire::{Status, ASK, LEN};

const SERVICE: &[u8] = b"tool.model-fetch";

pub(super) fn call(wait_ms: u64) -> Option<Status> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid);
    if rc < 0 || pid == 0 || port == 0 {
        return None;
    }
    let mut buf = [0u8; LEN];
    let n = mk_ipc_call_timeout(
        port as u64,
        ASK.as_ptr(),
        ASK.len(),
        buf.as_mut_ptr(),
        buf.len(),
        wait_ms,
    );
    if n < LEN as i64 {
        return None;
    }
    Status::decode(&buf)
}
