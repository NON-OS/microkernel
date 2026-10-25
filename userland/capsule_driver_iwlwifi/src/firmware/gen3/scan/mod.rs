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

//! The UMAC scan request, version 17 (`iwl_scan_req_umac_v17`, 1940 bytes),
//! filled as Linux v6.12 mvm/scan.c `iwl_mvm_scan_umac_v14_and_above` fills
//! it for a regular scan with no SSIDs while unassociated: forced passive (no
//! probe request is ever sent, so no address and no network name goes on the
//! air), pass all results, adaptive dwell, one iteration, the unassociated
//! timing (no time off channel limits), link 0 as the scan's reference, each
//! channel with its band in the top flag bits. EBS (energy-based scan) is off,
//! which Linux also does after an EBS failure. The probe parameters stay zero.
//! The completion (`iwl_umac_scan_complete`) is read back by uid.

mod complete;
mod consts;
mod request;

pub use complete::complete;
pub use consts::*;
pub use request::request;
