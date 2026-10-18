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

//! The ranges firmware keeps for itself, from the motherboard resource
//! devices, published to the broker. The wire is the kernel's own file,
//! mounted by path, so the capsule cannot send a list the kernel reads
//! differently.

mod publish;
#[path = "../../../../src/hardware/broker/reserved/wire.rs"]
#[allow(dead_code)] // `decode` and `end` are the kernel's to use
mod wire;

pub use publish::publish;
