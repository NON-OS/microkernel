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

//! Which failed I/O commands go again. A controller sets Do Not Retry when
//! the same command would fail the same way; without it the failure may be
//! transient (an internal error, a namespace not yet ready, a path that
//! came back), and Linux sends the command again. A media error is not
//! retried even then: the drive has already retried it internally, often
//! for seconds, and the client's own deadline is near.

use super::constants::{COMPLETION_TIMEOUT_MS, REPLY_BY_MS};
use super::queue::IoQueue;
use crate::clock;
use crate::error::{NvmeError, NvmeResult};

/// Times a failed command is sent again.
pub const RETRIES: u32 = 2;

/// How far into a request a failed command may still go again, counted
/// from the request's start: a retry waits a whole COMPLETION_TIMEOUT_MS of
/// its own, which then still ends by REPLY_BY_MS.
pub(super) const RETRY_WINDOW_MS: u64 = REPLY_BY_MS - COMPLETION_TIMEOUT_MS;

/// Whether a completion with status field `status` may be sent again: Do
/// Not Retry (bit 15) clear, and not a media and data integrity error
/// (status code type 2). Pure, so the host proofs hold it.
pub const fn worth_retrying(status: u16) -> bool {
    status & 0x8000 == 0 && (status >> 9) & 0x7 != 2
}

impl IoQueue {
    /// Run `op`, sending it again at most RETRIES times while its error
    /// completion allows and the clock reads before `retry_until`.
    pub(super) fn tries<F>(&mut self, mut op: F, retry_until: u64) -> NvmeResult<()>
    where
        F: FnMut(&mut IoQueue) -> NvmeResult<()>,
    {
        let mut tries = 0;
        loop {
            match op(self) {
                Err(NvmeError::AdminCommandFailed)
                    if tries < RETRIES
                        && worth_retrying(self.last().status)
                        && clock::uptime_ms().is_some_and(|now| now < retry_until) =>
                {
                    tries += 1
                }
                done => return done,
            }
        }
    }
}
