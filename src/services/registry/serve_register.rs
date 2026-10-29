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

//! A Linux run's served endpoint, registered for it and held to the capsule
//! its SERVES row names (serve_rule.rs, held_gate.rs).

use alloc::string::String;

use super::error::RegError;
use super::serve_rule::serves::SERVES;
use super::serve_rule::{served_endpoint, serves_none};
use super::{auth, required_caps, ServiceEndpoint, ENDPOINTS, MAX_SERVICES};
use crate::capabilities::Capability;

/// Register SERVES row `entry` for `pid`, held to its capsule, and give its
/// port. Exists when the name is served, or when `pid` serves one already:
/// both are read under the one lock, so two threads of one run cannot each
/// take one. It goes with the process, as every endpoint the process owns
/// does (process/exit/finalize.rs).
pub(crate) fn register_served(entry: usize, pid: u32) -> Result<u32, RegError> {
    let (name, port) = served_endpoint(entry);
    let caps = required_caps(&name, Capability::IPC.bit());
    if !auth::owner_has_required(pid, caps) {
        return Err(RegError::PermissionDenied);
    }
    let mut eps = ENDPOINTS.lock();
    let serving = !serves_none(eps.iter().filter(|e| e.pid == pid).map(|e| e.name.as_str()));
    if serving || eps.iter().any(|e| e.name == name || e.port == port) {
        return Err(RegError::Exists);
    }
    if eps.len() >= MAX_SERVICES {
        return Err(RegError::Full);
    }
    let mut ep = ServiceEndpoint::new(&name, port, pid, caps);
    ep.held_to = Some(String::from(SERVES[entry].0));
    eps.push(ep);
    Ok(port)
}
