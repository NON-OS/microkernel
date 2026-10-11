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

/*
 * The wallet's side of the service's one job. A press starts a job and
 * returns; each tick asks for its result until it is done or failed, then
 * hands the answer to `apply`. While a job runs no second one is started,
 * and the screens say what is being waited on.
 */

use alloc::string::String;

use shield_wire::field;

pub use super::job_poll::poll;
pub use super::job_resume::resume;
pub use super::job_start::start;
pub use super::job_steps::cancel;

pub(super) const BUSY: &str = "The shield service is still busy with the last request.";

pub(super) fn why(body: &str) -> String {
    String::from(field(body, "why").unwrap_or("The shield service refused that."))
}
