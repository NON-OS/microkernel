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

//! The client policy, `nsh-client-policy/1`, as the checks read it: the
//! fleet, the enrolment authorities, the accepted releases and the revoked
//! enrolments. Reading the signed policy file is the client's; this is the
//! policy once its signature has been checked, and `Policy::new` refuses
//! one that could never admit a host or could admit an unmeasured one.

mod build;
mod lookup;
mod types;

pub use types::{Authority, Policy, PolicyError, Release};

/// Most releases one policy accepts at a time.
pub const MAX_RELEASES: usize = 64;
/// Most enrolment authorities one fleet has.
pub const MAX_AUTHORITIES: usize = 16;
/// Most revoked enrolments one policy lists.
pub const MAX_REVOKED: usize = 65_536;
