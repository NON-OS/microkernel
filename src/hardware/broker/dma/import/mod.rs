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

//! A client surface's frames lent to a display device for direct scanout
//! (a project decision of 2026-11-02): `MkDmaImportSurface` maps them read only
//! into the IOMMU domain of a device the caller has claimed, at one IOVA run,
//! and `MkDmaUnimportSurface` takes them out, the IOTLB invalidation waited
//! on, before it returns.
//!
//! Refused without an IOMMU in service for the device: there is no physical
//! address fallback. Refused for a surface the caller does not hold attached:
//! the attach is what keeps the frames allocated, and every path that drops
//! it (a release of the attach, the holder's exit) takes the import out
//! first. A device release, a revoke and the holder's exit drain its imports
//! as they drain its DMA grants. An unmap the unit does not confirm leaves
//! the frames reachable: they are quarantined and never reused.
//!
//! Invariant (formal model name `scanout_import_read_only`): a frame is in a
//! device's domain through an import only while a live record names it, only
//! read only, and only while its importer holds the surface attached.

mod drop;
mod import;
mod records;
pub(crate) mod rule;
pub(crate) mod runs;
pub(crate) mod table_rule;
mod unimport;

pub(in crate::hardware::broker) use drop::drop_for_device;
pub use drop::{drop_for_attach, drop_for_handle, drop_for_pid};
pub use import::import_surface;
pub use rule::{ImportError, ImportResult};
pub use table_rule::UnimportError;
pub use unimport::unimport_surface;
