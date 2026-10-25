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

use crate::kernel_core::process_spawn::capsule_spawn::SpawnError;

// Why a runtime capsule load failed inside the loader. Spawn carries the verdict
// from the existing verified spawn path, so a store-sourced capsule is rejected
// for exactly the same reasons a baked one is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadError {
    Manifest,
    TrustAnchor,
    /// An endpoint name the image reserves, or outside the publisher's own.
    ReservedName,
    Spawn(SpawnError),
    /// The capsule would hold capabilities its parent lacks, and the parent
    /// is no launcher.
    BeyondParent,
}
