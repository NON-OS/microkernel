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

//! Stopping a scan early. A join hunts its network with a passive sweep and
//! stops it once the network's beacon is in hand, so the channel the join
//! needs is not held by a sweep still walking the band.
//!
//! `struct iwl_umac_scan_abort` (fw/api/scan.h), version 1, 8 bytes: `uid` 0,
//! `flags` 4. Filled as Linux v6.12 mvm/scan.c `iwl_mvm_umac_scan_abort`
//! fills it: the scan's uid, no flags. The firmware then sends the scan's
//! completion with status `IWL_SCAN_OFFLOAD_ABORTED` (2), which the sweep
//! takes like any completion.

use super::super::scan::SCAN_UID;

pub const SCAN_ABORT_LEN: usize = 8;

/// `iwl_umac_scan_abort` for the one scan this driver runs.
pub fn scan_abort() -> [u8; SCAN_ABORT_LEN] {
    let mut c = [0u8; SCAN_ABORT_LEN];
    c[0..4].copy_from_slice(&SCAN_UID.to_le_bytes());
    c
}
