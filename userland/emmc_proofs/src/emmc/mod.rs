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

//! The driver tree as the capsule builds it, without `platform`.

#[path = "../../../capsule_driver_ahci/src/emmc/disk/mod.rs"]
pub mod disk;
#[path = "../../../capsule_driver_ahci/src/emmc/env/mod.rs"]
pub mod env;
#[path = "../../../capsule_driver_ahci/src/emmc/error/mod.rs"]
pub mod error;
#[path = "../../../capsule_driver_ahci/src/emmc/info/mod.rs"]
pub mod info;
#[path = "../../../capsule_driver_ahci/src/emmc/mmc/mod.rs"]
pub mod mmc;
#[path = "../../../capsule_driver_ahci/src/emmc/pci/mod.rs"]
pub mod pci;
#[path = "../../../capsule_driver_ahci/src/emmc/sdhci/mod.rs"]
pub mod sdhci;
#[path = "../../../capsule_driver_ahci/src/emmc/text/mod.rs"]
pub mod text;
