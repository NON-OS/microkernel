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

//! IORT (Arm DEN0049, the IO Remapping Table): the ITS groups, SMMUv1/v2s
//! and SMMUv3s, root complexes, named components and reserved memory
//! ranges, and how each one's IDs map onto the next node's until they
//! reach an ITS.

mod kind;
mod mapping;
mod node;
mod node_data;
mod node_smmu;
mod rmr;
mod translate;
mod walk;

pub use mapping::Mapping;
pub use kind::Irq;
pub use node::{Kind, Node};
pub use rmr::{reserved, Reserved};
pub use translate::translate;
pub use walk::iort;
