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

//! What this capsule serves: the disk on a SATA port, or the eMMC of an
//! Atom, Celeron or Pentium Silver laptop. driver.ahci0 also serves eMMC
//! hosts until the eMMC driver gets publisher keys of its own; the eMMC
//! code is the self-contained `emmc` tree, so it moves to its own capsule
//! with the directory.

mod bring_up;
mod hosts;
mod kind;

pub use bring_up::bring_up;
pub use hosts::Hosts;
pub use kind::Served;
