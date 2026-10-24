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

//! The SMC Calling Convention (Arm DEN0028): the one way the kernel calls
//! firmware, by the conduit PSCI's device-tree node named, and SMCCC 1.1's
//! discovery of the architectural workarounds.

mod call;
mod discover;
mod ids;

pub use call::call;
pub use discover::{arch_features, version};
pub use ids::{ARCH_WORKAROUND_1, ARCH_WORKAROUND_2, ARCH_WORKAROUND_3};
