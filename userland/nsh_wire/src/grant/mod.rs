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

//! The role grant, `nsh-grant/1`, and the approvals an escalation carries.

pub(crate) mod approval;
mod parse;
pub(crate) mod rules;
pub(crate) mod types;

pub use approval::{Approval, APPROVAL_LEN, MAX_APPROVALS};
pub use types::{Grant, GrantKind, GRANT_ID_LEN, GRANT_MAGIC, GRANT_VERSION};
