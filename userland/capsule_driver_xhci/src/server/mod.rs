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
pub mod context;
mod dispatch;
mod error;
// The proofs reach the control data stage's region through it.
pub(crate) mod handlers;
pub mod mux;
mod mux_address;
mod mux_admit;
mod mux_at;
mod mux_auth;
mod mux_bind;
mod mux_claims;
mod mux_config;
mod mux_enable;
mod mux_entry;
mod mux_forward;
mod mux_held;
mod mux_list;
mod mux_park;
mod mux_police;
mod mux_port_status;
pub(crate) mod mux_ports;
mod mux_revived;
mod mux_routed;
mod mux_shared_reply;
mod mux_slots;
mod mux_status;
mod mux_sweep;
mod mux_virt;
pub(crate) mod reply;
mod route;
pub(crate) mod runner;
mod sender_holds;
mod served;
mod service_interrupts;
mod sweep;
mod vouch;
pub use runner::run;
