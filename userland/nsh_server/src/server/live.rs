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

//! The session once AUTH_OK is sent: the login, its record, its clocks,
//! and the escalations the host has been asked to start or that run.

use crate::record::Record;
use nsh_roles::{Escalation, Login};

/// The session once `AUTH_OK` is sent.
pub(crate) struct Live {
    pub login: Login,
    pub record: Record,
    pub last_input_ms: u64,
    /// The shell was asked to stop, for this reason.
    pub stopping: Option<crate::EndReason>,
    /// The client sent `CLOSE`.
    pub closing: bool,
    /// An escalation the host has been asked to start.
    pub pending: Option<Escalation>,
    /// The escalation that runs.
    pub active: Option<Escalation>,
}
