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

//! The scan request's sizes, flags, dwells and offsets.

/// The request's size.
pub const SCAN_REQ_V17_LEN: usize = 1940;
/// `SCAN_MAX_NUM_CHANS_V3`.
pub const MAX_CHANNELS: usize = 67;
/// The uid of the one scan this driver runs at a time.
pub const SCAN_UID: u32 = 0;

pub(super) const GEN_FLAGS_PASS_ALL: u16 = 1 << 1;
pub(super) const GEN_FLAGS_ADAPTIVE_DWELL: u16 = 1 << 7;
pub(super) const GEN_FLAGS_FORCE_PASSIVE: u16 = 1 << 11;
pub(super) const SCAN_PRIORITY_EXT_6: u32 = 6;
pub(super) const DWELL_ACTIVE: u8 = 10;
pub(super) const DWELL_PASSIVE: u8 = 110;
pub(super) const ADWELL_LB_N_APS: u8 = 2;
pub(super) const ADWELL_HB_N_APS: u8 = 8;
pub(super) const ADWELL_N_APS_SOCIAL: u8 = 10;
pub(super) const ADWELL_MAX_BUDGET_FULL: u16 = 300;
pub(super) const CHANNEL_FLAG_ENABLE_CHAN_ORDER: u8 = 1 << 5;
pub(super) const N_APS_GO_FRIENDLY: u8 = 10;
pub(super) const N_APS_SOCIAL_CHS: u8 = 2;
pub(super) const BAND_POS: u32 = 30;
pub(super) const PHY_BAND_5: u32 = 0;
pub(super) const PHY_BAND_24: u32 = 1;

// Offsets inside the request.
pub(super) const GENERAL: usize = 8;
pub(super) const CHANNELS: usize = GENERAL + 36;
pub(super) const CHANNEL_CFG: usize = CHANNELS + 4;
pub(super) const PERIODIC: usize = CHANNELS + 540;
