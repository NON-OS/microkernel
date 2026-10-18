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
//! Commands on the command ring, each run to its completion or aborted.
mod endpoint_at;
mod give_up;
pub(crate) mod issue_address_device;
mod issue_configure_endpoint;
mod issue_disable_slot;
mod issue_enable_slot;
mod issue_evaluate_context;
mod issue_noop_and_wait;
mod next_completion;
pub(crate) mod recover_endpoint;
pub(crate) mod recovered;
pub(crate) mod reset_toggle;
pub(crate) mod run_command;
mod settle_abort;
mod to_stopped;
pub(crate) mod wait_command_completion;
pub use endpoint_at::EndpointAt;
pub use issue_address_device::issue_address_device;
pub use issue_configure_endpoint::issue_configure_endpoint;
pub use issue_disable_slot::issue_disable_slot;
pub use issue_enable_slot::issue_enable_slot;
pub use issue_evaluate_context::issue_evaluate_context;
pub use issue_noop_and_wait::issue_noop_and_wait;
pub use recover_endpoint::recover_endpoint;
pub use recovered::Recovered;
