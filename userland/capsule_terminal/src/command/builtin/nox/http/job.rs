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

//! `http <url>` as a terminal job: the request is carried a step a tick
//! (`mixnet::exchange`), the window keeps painting, and Ctrl+C ends it with
//! the connection closed and the prompt back.

#[path = "job_prepare.rs"]
mod job_prepare;
#[path = "job_step.rs"]
mod job_step;

pub use job_prepare::prepare;

use crate::mixnet::{Exchange, Stage};

pub struct HttpJob {
    exchange: Exchange,
    /// The stage last told to the person, so each is said once.
    shown: Option<Stage>,
}
