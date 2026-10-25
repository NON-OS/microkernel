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

//! Launching an app, or raising the window it already has.

mod focus;
mod frame;
mod outcome;
mod refusal;

pub use focus::focus_app;
pub(crate) use frame::{focus_frame, lookup_pid};
pub use outcome::{request, request_service, LaunchOutcome};
pub use refusal::take_refusal;
