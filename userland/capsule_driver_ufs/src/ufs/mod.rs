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

//! The UFS engine: host controller, transfer requests, UPIUs, device
//! management, SCSI and the units, over `env`, so the capsule and the host
//! proofs run the same code.

pub mod bring_up;
pub mod ctx;
pub mod device;
pub mod disk;
pub mod env;
pub mod error;
pub mod facts;
pub mod host;
pub mod scsi;
pub mod slot;
pub mod text;
pub mod upiu;
