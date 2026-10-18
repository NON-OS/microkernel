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

//! Device dependencies (_DEP, ACPI 6.5, 6.5.8): what a device waits on
//! before an operating system should use it, a camera sensor on its PMIC,
//! a touchpad on its GPIO controller. `read` takes a device's list from
//! the namespace; `order` puts devices after what they depend on and
//! flags what cannot be satisfied, without hiding anything.

mod order;
mod read;

pub use order::{order, Placed, Verdict};
pub use read::{dependencies, Dep};
