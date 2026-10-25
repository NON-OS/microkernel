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

//! Host proofs for the UFS driver (`capsule_driver_ufs/src/ufs`), its engine
//! mounted by `#[path]`, run against `model`: a register level UFSHCI host
//! with a UFS device and its logical units behind it.

/// The hosts the capsule binds.
#[path = "../../capsule_driver_ufs/src/platform/hosts.rs"]
pub mod hosts;
#[path = "../../capsule_driver_ufs/src/ufs/mod.rs"]
pub mod ufs;

#[cfg(test)]
mod model;
#[cfg(test)]
mod tests;
