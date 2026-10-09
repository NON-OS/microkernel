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

// The real controller register snapshot and the checks bring-up makes on it.
// The register reads themselves are MMIO and stay out of the host build.
#[path = "../../../capsule_driver_nvme/src/controller/info/doorbell_stride.rs"]
mod doorbell_stride;
#[path = "../../../capsule_driver_nvme/src/controller/info/doorbells_fit.rs"]
mod doorbells_fit;
#[path = "../../../capsule_driver_nvme/src/controller/info/info_type.rs"]
mod info_type;

pub use info_type::ControllerInfo;
