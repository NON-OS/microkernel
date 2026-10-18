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

mod acl;
mod address;
mod alive;
mod answer;
mod bringup;
mod command;
mod connect;
mod controller;
mod encrypt;
mod events;
mod features;
mod held;
mod length;
mod lost;
mod ltk;
mod masks;
mod new;
mod open;
mod pending;
mod reach;
mod scan;
mod state;
mod update;
mod wait;

pub use alive::READ_RSSI;
pub use answer::{answer, Answer};
pub use connect::{create_connection_params, create_connection_params_as, Connect, DEFAULT_LINK};
pub use controller::Controller;
pub use events::QUEUE_MAX;
pub use length::ASK_TIME_US;
pub use ltk::enable_encryption_params;
pub use masks::{EVENT_MASK, LE_EVENT_MASK};
pub use scan::{Scan, DEFAULT_SCAN};
pub use state::{Host, COMMAND_CREDITS_MAX, POLL_BUDGET};
pub use update::connection_update_params;
