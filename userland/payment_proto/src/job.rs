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

//! The meter's jobs (plan/21, v1.4.14). OPEN START, TOPUP and CLOSE wait on
//! settle and the network for up to minutes, so each answers at once with a
//! job id (u32 LE), or EBUSY while another job runs. JOB (op 15) takes the
//! id and answers EAGAIN while the job runs, then the job's own reply, or
//! its status, once. A job id is never reused within a boot, so a stale or
//! taken id answers ENOENT, never another job's result. Results live in
//! memory only: after a reboot the wallet rechecks the chain.

pub const JOB_ID_LEN: usize = 4;

pub fn job_id(id: u32) -> [u8; JOB_ID_LEN] {
    id.to_le_bytes()
}

pub fn parse_job_id(b: &[u8]) -> Option<u32> {
    Some(u32::from_le_bytes(b.try_into().ok()?))
}
