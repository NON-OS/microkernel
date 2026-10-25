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

/*
 * A device behind a bridge that may stand in for its requester id is never
 * counted confined (L51). The kernel's rule and walk are included by path
 * and fed made-up topologies: a laptop's root ports and a switch, a PCIe to
 * PCI bridge, a conventional bridge, a missing parent and a numbering loop.
 */

#[path = "../../../../src/arch/x86_64/iommu/own_id.rs"]
mod own_id;
mod root_bus_tests;
mod tests;
