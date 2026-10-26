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

#[path = "../../../../../src/hardware/block_device/error.rs"]
mod error;

// Reached by pass_tests only.
#[path = "../../../../../src/hardware/block_device/pass.rs"]
#[allow(dead_code)]
mod pass;
#[path = "../../../../../src/hardware/block_device/wait_since.rs"]
#[allow(dead_code)]
mod wait_since;

pub use error::BlockDeviceError;

#[cfg(test)]
mod pass_tests;

// Who the block clients serve: a driver, the store's holder, or the data
// volume's own I/O.
#[cfg(test)]
mod volume_gate_glue;
#[cfg(test)]
mod volume_gate_tests;
#[cfg(test)]
#[path = "../../../../../src/hardware/block_device/volume_rule.rs"]
mod volume_rule;
