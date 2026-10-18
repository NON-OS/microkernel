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

//! The EDID parser, proved without a monitor.
//!
//! The `#[path]` includes pull in the shipping parser, at the module names
//! its `crate::` paths use, so these tests run the code every display
//! capsule and the loader read panels with. The EDIDs are built byte by
//! byte here from the layouts in E-EDID 1.4, CTA-861 and DisplayID 2.0.

#[path = "../../nonos_edid/src/base/mod.rs"]
pub mod base;
#[path = "../../nonos_edid/src/blocks.rs"]
pub mod blocks;
#[path = "../../nonos_edid/src/cta/mod.rs"]
pub mod cta;
#[path = "../../nonos_edid/src/displayid/mod.rs"]
pub mod displayid;
#[path = "../../nonos_edid/src/edid.rs"]
pub mod edid;
#[path = "../../nonos_edid/src/error.rs"]
pub mod error;
#[path = "../../nonos_edid/src/query.rs"]
pub mod query;
#[path = "../../nonos_edid/src/source.rs"]
pub mod source;
#[path = "../../nonos_edid/src/timing.rs"]
pub mod timing;
#[path = "../../nonos_edid/src/walk.rs"]
pub mod walk;

/// The loader's own base block parse, which the crate must agree with
/// until the loader mounts the crate in its place.
#[path = "../../../nonos-bootloader/src/display/gop/pick.rs"]
pub mod loader_pick;

#[cfg(test)]
mod build;
#[cfg(test)]
mod build_did;
#[cfg(test)]
mod build_dtd;
#[cfg(test)]
mod build_ext;
#[cfg(test)]
mod tests;
