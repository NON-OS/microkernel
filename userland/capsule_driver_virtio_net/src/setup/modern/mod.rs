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

//! The virtio 1.0 bring-up, for any function presenting the virtio 1.0
//! structures (`transport::choose`): a transitional virtio-net with its
//! legacy register window too, and a modern-only one, as QEMU builds
//! behind its IOMMU.

mod config;
mod program;
mod queues;
mod run;
mod spec;

pub use run::run;
