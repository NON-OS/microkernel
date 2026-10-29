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

//! Dropping endpoints: every one a process owns, when it ends
//! (process/exit/finalize.rs), or one by name.

use super::ENDPOINTS;

pub fn unregister_endpoints_for_pid(pid: u32) -> usize {
    let mut eps = ENDPOINTS.lock();
    let before = eps.len();
    eps.retain(|e| e.pid != pid);
    before - eps.len()
}

/// Drop the endpoint registered under `name`, whoever owns it.
pub fn unregister_endpoint_by_name(name: &str) -> bool {
    let mut eps = ENDPOINTS.lock();
    let before = eps.len();
    eps.retain(|e| e.name != name);
    before != eps.len()
}
