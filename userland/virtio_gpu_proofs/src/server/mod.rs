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
//! The driver's server half, less the loop that waits on the kernel's IPC:
//! its handlers, its replies and its dispatch, which these tests call with
//! the request a compositor would send.

#[path = "../../../capsule_driver_virtio_gpu/src/server/runner/dispatch.rs"]
pub mod dispatch;
#[path = "../../../capsule_driver_virtio_gpu/src/server/handlers/mod.rs"]
pub mod handlers;
#[path = "../../../capsule_driver_virtio_gpu/src/server/respond.rs"]
pub mod respond;
