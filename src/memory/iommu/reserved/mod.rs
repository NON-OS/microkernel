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

//! Ranges a device must find mapped through its IOMMU from the moment it is
//! attached: firmware's RMRRs on x86_64, the device tree's
//! `iommu-addresses` on aarch64. The type, the hook's answer and the checks
//! are pure, so the proofs run them.

mod admit;
mod answer;
mod range;
mod refusal;

pub use admit::admit;
pub use answer::{Reserved, MAX_RESERVED};
pub use range::ReservedRange;
pub use refusal::Refusal;
