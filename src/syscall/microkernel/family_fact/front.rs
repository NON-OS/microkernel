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

//! Who may ask MkFamilyFact: the processes registered for the anonymity
//! fronts, holding Network.

use crate::capabilities::Capability;
use crate::services::registry::{lookup_service, ANON_FRONTS};

pub(super) fn caller_is_front() -> bool {
    let Some(caller) = crate::process::current_pid() else { return false };
    crate::process::caps::has(caller, Capability::Network.bit())
        && ANON_FRONTS.iter().any(|name| lookup_service(name).is_some_and(|ep| ep.pid == caller))
}
