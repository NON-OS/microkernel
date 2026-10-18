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

/* A meter job the wallet started (v1.4.14), and its result in words. */

use alloc::string::String;

use nonos_payment_proto::EAGAIN;

/* Mounted by path so wallet_proofs reads the same sentences. */
#[path = "job_said.rs"]
mod said;
#[path = "job_sent.rs"]
mod sent;

pub use sent::closed_due;
use sent::sent;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum JobKind {
    TopUp,
    Close,
    /// The NOX the Shield pool credited the meter (op 16, v1.4.17).
    Claim,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MeterJob {
    pub id: u32,
    pub kind: JobKind,
    /// A close's channel's reclaimTo (v1.4.60), for the due list once sent.
    pub reclaim_to: Option<[u8; 20]>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Outcome {
    Running,
    Sent(String),
    Failed(&'static str),
}

pub fn outcome(kind: JobKind, r: Result<&[u8], i32>) -> Outcome {
    match r {
        Err(EAGAIN) => Outcome::Running,
        Ok(b) => sent(kind, b).map_or(
            Outcome::Failed("The meter's answer could not be read; check the channel."),
            Outcome::Sent,
        ),
        Err(s) => Outcome::Failed(said::failed(kind, s)),
    }
}
