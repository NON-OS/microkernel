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

//! A hydrate pass run against a stand-in policy service, for the proofs.

use crate::ipc::error::IpcError;
use crate::ipc::hydrate_pass::{hydrate_fields, HYDRATE_RETRY_MS};

/// The status strip's buffer (`Status::MAX_MSG` in the capsule).
pub(crate) const STATUS_CAP: usize = 96;

/// Run the pass over `fields` with `answer` standing in for the policy
/// service; return what it stored, how many reads it made and what it reported.
pub(crate) fn run(
    fields: &[u32],
    answer: impl Fn(u32) -> Result<u32, IpcError>,
) -> (Vec<(u32, u32)>, usize, Option<IpcError>) {
    let mut stored = Vec::new();
    let mut reads = 0;
    let get = |field| {
        reads += 1;
        answer(field)
    };
    let err = hydrate_fields(fields, get, |field, value| stored.push((field, value)));
    (stored, reads, err)
}

/* Soon enough to replace the defaults while the window is still being read. */
const _: () = assert!(HYDRATE_RETRY_MS > 0 && HYDRATE_RETRY_MS <= 5_000);
