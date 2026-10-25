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

//! What setup settles a bind on, its sweep for a pad declared with no
//! address, the order and speed it tries controllers in, and the bring-up
//! of one it has claimed and mapped, as shipped. Claiming and mapping go
//! through the broker and stay out; the bench maps the modelled core in
//! their place, and the device list is the shim's, which the tests fill.

#[path = "../../capsule_driver_i2c_pci/src/setup/sequence/brought_up.rs"]
pub mod brought_up;
#[path = "../../capsule_driver_i2c_pci/src/setup/sequence/order.rs"]
pub mod order;
#[path = "../../capsule_driver_i2c_pci/src/setup/sequence/settle.rs"]
pub mod settle;
#[path = "../../capsule_driver_i2c_pci/src/setup/sequence/sweep.rs"]
pub mod sweep;
#[path = "../../capsule_driver_i2c_pci/src/setup/sequence/unlisted_gate.rs"]
mod unlisted_gate;
