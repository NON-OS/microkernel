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
//! Parts of what the Shield screens remember, each reset with the rest when
//! ShieldUi is: the service's answering, the running job, and the payee.

use alloc::string::String;

#[derive(Default)]
pub struct ServiceLink {
    /* The service did not answer in time, as one still starting on a slow
     * machine may not: opening is asked again by itself at this uptime, with
     * a longer wait each time, and `tries` says how many times so far. */
    pub retry_at_ms: Option<i64>,
    pub tries: u8,
    /* The service keeps this store in memory, for this boot only. */
    pub live: bool,
    /* Result asks in a row the service did not answer, and the uptime of
     * the next one: a service that stops answering is asked less often,
     * and after `job_steps::MISSES` the job is said lost. */
    pub misses: u8,
    pub poll_at_ms: i64,
    pub last_follow_ms: i64,
    pub last_sync_ms: i64,
}

#[derive(Default)]
pub struct JobProgress {
    /* Seconds the running job has taken, as of the last tick. */
    pub elapsed_s: u32,
    /* The prover's own phase and how far through it, in thousandths, when
     * the service reports them. */
    pub phase: Option<String>,
    pub permille: Option<u16>,
}

#[derive(Default)]
pub struct Payee {
    /* The public address a withdrawal pays: a fresh one of this wallet, or
     * the meter's when `to_meter` (B7). */
    pub withdraw_to: Option<String>,
    pub to_meter: bool,
    /* An escrow order's deposit address, when the withdrawal funds one (B8). */
    pub to_deposit: Option<[u8; 20]>,
}
