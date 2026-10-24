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

//! The nsh/1 role table and the checks a host runs on a grant
//! (docs/design/nsh-v1-roles.md). The host installs a signed role table,
//! `nsh-policy/1`, and a signed revocation list, `nsh-revoke/1`; both are
//! trusted only once their signatures verify. A login is then checked in the
//! order the specification gives, and so is an escalation. Each failed check
//! is its own typed `Refusal`, and each maps to the one reason code the
//! peer is told. Every time is u64 milliseconds the caller passes in from
//! the host clock; nothing here reads a clock.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

mod cap_set;
pub mod caps;
mod check;
mod error;
mod hybrid;
mod policy;
mod reason;
mod refusal;
mod revoke;
mod session;
pub mod shipped;

pub use cap_set::{CapError, CapSet};
pub use check::approvals::{approvals, MAX_ESCALATION_MS};
pub use check::escalate::escalate;
pub use check::login::login;
pub use check::Host;
pub use error::PolicyError;
pub use hybrid::{key_id, verify, Key};
pub use policy::{open_policy, Policy, Role, Table, Unverified};
pub use policy::{MAX_APPROVERS, MAX_AUTHORITIES, MAX_REVOCATION_AGE_MS, MAX_ROLES};
pub use policy::{MAX_SPAN_MS, POLICY_MAGIC};
pub use refusal::Refusal;
pub use revoke::{open_revocations, RevocationList, Revocations, MAX_REVOKED, REVOKE_MAGIC};
pub use session::{grant_hash, Escalation, Login};
