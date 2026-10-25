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

//! Whether a name may be asked in the clear: only on the Direct route.
//!
//! net.sockets refuses a host lookup off Direct, but net.core answers
//! RESOLVE_A to any caller holding the Network capability, the terminal's
//! `ping` and `nox` among them, so the rule is held here too. The choice is
//! read afresh for each lookup; a policy store that does not answer is not
//! Direct, as every other reader of the choice takes it.

use nonos_policy_proto::route;

/// The rule, pure: `chosen` is the NetworkRoute the policy store gave.
pub fn clear_lookup_allowed(chosen: Option<u8>) -> bool {
    chosen == Some(route::DIRECT)
}
