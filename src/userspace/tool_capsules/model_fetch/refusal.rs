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

/* Why the fetcher did not start, as the errno the Terminal names. */

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;
use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_NETDOWN, ERRNO_NOMEM, ERRNO_PERM};

pub(super) fn errno(e: SpawnError) -> i64 {
    match e {
        /*
         * Its endpoint is registered: one fetcher runs already, for another
         * `qwen get` or `qwen tiers`.
         */
        SpawnError::EndpointCollision => ERRNO_BUSY,
        SpawnError::ProfileRefused => ERRNO_NETDOWN,
        SpawnError::NonosIdCertRejected(_)
        | SpawnError::ManifestRejected(_)
        | SpawnError::AttestationRejected => ERRNO_PERM,
        _ => ERRNO_NOMEM,
    }
}
