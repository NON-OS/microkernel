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

//! How long the extras may take, when they are asked for at all, and which
//! of their failures end the attempt.

use crate::error::NvmeError;

/// How long SET FEATURES Host Memory Buffer may take. A controller may
/// copy its tables into the buffer before it answers; Linux gives every
/// admin command a minute (NVME_ADMIN_TIMEOUT). Half that, and the
/// identify budget for the rest.
pub const ENABLE_TIMEOUT_MS: u64 = 30_000;
/// SET FEATURES Number of Queues answers at once; the identify budget.
pub const QUEUES_TIMEOUT_MS: u64 = 5_000;

/// Whether an attempt may ask for the extras (Number of Queues, the host
/// memory buffer). Every attempt may until one that asked for them fails;
/// from then on attempts run as they did before the extras existed, so a
/// controller that mishandles them costs one attempt, never the disk.
pub const fn extras_allowed(an_attempt_with_extras_failed: bool) -> bool {
    !an_attempt_with_extras_failed
}

/// Whether an extra's SET FEATURES failure ends the attempt. An error
/// completion is the controller saying no, and the drive is served without
/// it; no answer, or no clock to time one, means the controller cannot be
/// trusted with the next command.
pub const fn ends_attempt(e: NvmeError) -> bool {
    !matches!(e, NvmeError::AdminCommandFailed)
}
