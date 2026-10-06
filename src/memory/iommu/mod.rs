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

mod backend;
mod capabilities;
mod device;
mod domain;
mod domain_id;
mod error;
mod posture;
mod protection;
mod query;
mod unconfined;
mod vendor;

pub use capabilities::IommuCapabilities;
pub use device::DeviceAddress;
pub use domain::IommuDomain;
pub use domain_id::DomainId;
pub use error::IommuError;
pub use posture::report_posture;
pub use protection::IommuProtection;
pub use query::{capabilities, select_vendor, translates};
pub use unconfined::{note_unconfined, note_unconfined_released, unconfined_grants};
pub use vendor::IommuVendor;
