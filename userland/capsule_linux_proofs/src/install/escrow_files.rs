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

//! The escrow pin and the install role's check of a delivery against it,
//! which the capsule reads once the meter's hand-off lands (I-C5).

#[path = "../../../capsule_linux/src/linux/install/escrow_check.rs"]
pub mod escrow_check;
#[path = "../../../capsule_linux/src/linux/install/escrow_pin.rs"]
pub mod escrow_pin;
