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

//! What the serving thread hands the worker, and what it hands back
//! (worker.rs). Each input wipes its secrets when it drops.

use alloc::vec::Vec;

use super::device::Device;
use super::isolation::Isolation;

/// One job's proofs: the device's, then the isolation's for a PROVE.
pub struct Work {
    pub device: Device,
    pub isolation: Option<Isolation>,
}

/// The proofs, encoded; the isolation's empty for a DELIVER.
pub struct Made {
    pub device: Vec<u8>,
    pub isolation: Vec<u8>,
}
