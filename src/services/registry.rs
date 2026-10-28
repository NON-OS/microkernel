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

use alloc::vec::Vec;
use spin::Mutex;

mod addressed;
mod adopt;
mod auth;
mod endpoint;
mod error;
mod held;
pub(crate) mod nics;
mod peers;
mod peers_check;
mod peers_inbox;
mod policy;
mod reserved;
mod serve_register;
mod serve_rule;
mod system_names;
mod unregister;
pub(crate) mod usb_hosts;

pub(crate) use addressed::addressed_admits;
pub(crate) use adopt::adopt_endpoint;
pub(crate) use auth::caller_has_register_right;
pub use endpoint::ServiceEndpoint;
pub use error::RegError;
pub(crate) use held::displays;
pub use peers_check::caller_may_reach;
pub(crate) use peers_check::{caller_group, caller_owns};
pub use peers_inbox::{caller_may_reach_pid, caller_may_write_inbox};
pub use policy::required_caps;
pub(crate) use reserved::{is_reserved_service, is_runtime_registrable};
pub(crate) use serve_register::register_served;
pub(crate) use serve_rule::{admit_serve, is_serve_port, ServeRefusal, SERVE_NAME_MAX};
pub(crate) use system_names::{is_reserved_form, is_system_name};
pub use unregister::{unregister_endpoint_by_name, unregister_endpoints_for_pid};

pub const MAX_SERVICES: usize = 256;
pub(self) static ENDPOINTS: Mutex<Vec<ServiceEndpoint>> = Mutex::new(Vec::new());

/// Register a service endpoint on behalf of `pid`.
pub fn register_endpoint(name: &str, port: u32, pid: u32, caps: u64) -> Result<(), RegError> {
    if !auth::owner_has_required(pid, caps) {
        return Err(RegError::PermissionDenied);
    }
    let mut eps = ENDPOINTS.lock();
    if eps.iter().any(|e| e.name == name && e.port == port && e.pid == pid) {
        return Ok(());
    }
    if eps.iter().any(|e| e.name == name || e.port == port) {
        return Err(RegError::Exists);
    }
    if eps.len() >= MAX_SERVICES {
        return Err(RegError::Full);
    }
    eps.push(ServiceEndpoint::new(name, port, pid, caps));
    Ok(())
}

pub fn lookup_service(name: &str) -> Option<ServiceEndpoint> {
    ENDPOINTS.lock().iter().find(|e| e.name == name).cloned()
}

pub fn lookup_port(port: u32) -> Option<ServiceEndpoint> {
    ENDPOINTS.lock().iter().find(|e| e.port == port).cloned()
}

/// Every endpoint `pid` serves, its reply inbox included.
pub fn endpoints_of(pid: u32) -> Vec<ServiceEndpoint> {
    ENDPOINTS.lock().iter().filter(|e| e.pid == pid).cloned().collect()
}
