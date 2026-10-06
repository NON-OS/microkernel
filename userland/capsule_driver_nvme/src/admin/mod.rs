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

mod active_ns;
mod command;
mod completion;
mod completion_step;
mod completion_wait;
mod controller;
mod cq_cursor;
mod disable_step;
mod health;
pub mod hmb;
mod identity;
mod namespace;
mod queue;
mod ready_step;
mod ready_wait;

pub use active_ns::{first_active_nsid, lists_active_namespaces, FALLBACK_NSID};
pub use command::set_features;
pub use controller::{enable, reset_to_disabled};
pub use health::SmartHealth;
pub use identity::ControllerIdentity;
pub use namespace::NamespaceIdentity;
pub use queue::AdminQueue;

pub(crate) use command::Submission;
pub(crate) use completion::Completion;
pub(crate) use completion_wait::{wait_for_completion, wait_noting_foreign};
pub(crate) use cq_cursor::CqCursor;
