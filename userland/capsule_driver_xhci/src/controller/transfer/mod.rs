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
//! Transfers on a device's endpoints: control, interrupt-IN and bulk.
mod bulk;
mod get_device_descriptor;
mod issue_control_transfer;
mod poll_interrupt_in;
mod transfer_event;
pub(crate) mod wait_transfer_completion;
pub use bulk::{bulk_transfer, reset_bulk_endpoint, write_bulk_input, BulkEndpoint, BulkPipes};
pub use get_device_descriptor::{
    get_device_descriptor, read_device_descriptor, DEVICE_DESCRIPTOR_LEN, DEVICE_DESCRIPTOR_PREFIX,
};
pub use issue_control_transfer::{issue_control_transfer, ControlRequest};
pub use poll_interrupt_in::{poll_interrupt_in, IntrPoll};
