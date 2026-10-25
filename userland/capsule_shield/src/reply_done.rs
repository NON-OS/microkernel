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

//! A finished job's envelope, the reply RESULT gives.

use shield_wire::Values;

/// A finished job's reply to RESULT: the envelope, then the job's own values,
/// any named as the envelope's going as `job_<name>`.
pub fn done(name: &str, job: &str) -> Values {
    let mut v = Values::new();
    v.put("job", name).put("state", "done").put_job(job);
    v
}
