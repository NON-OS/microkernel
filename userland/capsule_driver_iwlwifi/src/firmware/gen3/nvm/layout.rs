// NONOS Operating System (AGPL-3.0-or-later)

//! Where the NVM reply keeps each field, and its channel list.

pub(super) const V4_LEN: usize = 468;
pub(super) const V3_LEN: usize = 8 + 4 + 8 + 4 + 51 * 2 + 2;
pub(super) const PROFILE_AT: usize = 20 + 8;
pub(super) const PROFILE_AT_V3: usize = 20 + 4;

/// `NVM_CHANNEL_VALID`.
pub(super) const CHANNEL_VALID: u32 = 1 << 0;

/// The 2.4 and 5 GHz part of `iwl_ext_nvm_channels` / `iwl_uhb_nvm_channels`
/// (identical up to there; the 6-7 GHz tail is not scanned).
pub const NVM_CHANNELS: [u8; 51] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 40, 44, 48, 52, 56, 60, 64, 68, 72, 76, 80, 84, 88,
    92, 96, 100, 104, 108, 112, 116, 120, 124, 128, 132, 136, 140, 144, 149, 153, 157, 161, 165, 169,
    173, 177, 181,
];
