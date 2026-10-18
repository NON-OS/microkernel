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

mod apps_desk;
mod apps_media;
mod apps_tools;
mod boot_frame;
mod error_name;
mod named;
mod priority;
mod queue;
mod request;
mod service;

pub(super) use priority::adopt as adopt_drain_pid;
pub(super) use priority::{raise as raise_drain, set as set_drain_priority};
pub(crate) use queue::has_pending;
pub use queue::PendingApp;
pub use request::request;
pub(crate) use service::service;
