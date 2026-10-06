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

//! Session protection: the time event that keeps the radio on the access
//! point's channel while the station authenticates, associates and runs the
//! handshake. This firmware has `IWL_UCODE_TLV_CAPA_SESSION_PROT_CMD`, so
//! Linux asks for it with SESSION_PROTECTION_CMD rather than TIME_EVENT_CMD.
//!
//! `struct iwl_mvm_session_prot_cmd` (fw/api/time-event.h),
//! SESSION_PROTECTION_CMD_API_S_VER_1 and _VER_2, 24 bytes: `id_and_color` 0
//! (the MAC in version 1, the link in version 2: 0 either way here),
//! `action` 4, `conf_id` 8, `duration_tu` 12, `repetition_count` 16,
//! `interval` 20. Filled as Linux v6.12 mvm/time-event.c
//! `iwl_mvm_schedule_session_protection` fills it before a join
//! (`iwl_mvm_mac_mgd_prepare_tx` asks for 900 ms of
//! `SESSION_PROTECT_CONF_ASSOC`), and as `iwl_mvm_cancel_session_protection`
//! removes it once the port opens. `MSEC_TO_TU` is ms * 1000 / 1024.
//!
//! `struct iwl_mvm_session_prot_notif`, SESSION_PROTECTION_NOTIF versions 2
//! and 3, 16 bytes: `mac_link_id` 0, `status` 4 (1 when the request was
//! scheduled), `start` 8 (1 when the session began, 0 when it ended),
//! `conf_id` 12.

/// `SESSION_PROTECT_CONF_ASSOC`.
pub const CONF_ASSOC: u32 = 0;
pub const SESSION_PROT_LEN: usize = 24;
const NOTIF_LEN: usize = 16;

/// The session Linux asks for before a join.
pub const JOIN_SESSION_MS: u32 = 900;

/// `MSEC_TO_TU`.
pub const fn tu_from_ms(ms: u32) -> u32 {
    ms.saturating_mul(1000) / 1024
}

/// `iwl_mvm_session_prot_cmd` for link 0: `duration_ms` of association
/// protection to add, or (with [`super::phy::ACTION_REMOVE`]) the one
/// running to cancel, with a duration of 0.
pub fn session_protection(action: u32, duration_ms: u32) -> [u8; SESSION_PROT_LEN] {
    let mut c = [0u8; SESSION_PROT_LEN];
    c[4..8].copy_from_slice(&action.to_le_bytes());
    c[8..12].copy_from_slice(&CONF_ASSOC.to_le_bytes());
    c[12..16].copy_from_slice(&tu_from_ms(duration_ms).to_le_bytes());
    c
}

/// What a session protection notification says.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Session {
    /// The firmware is on the channel for the session.
    Started,
    /// The session ended (its time ran out, or it was cancelled).
    Ended,
    /// The firmware could not schedule the session.
    Refused,
}

/// Read a session protection notification for link 0's association
/// session; `None` for one too short, or about another link or session.
pub fn parse_notif(payload: &[u8]) -> Option<Session> {
    if payload.len() < NOTIF_LEN {
        return None;
    }
    let word = |at: usize| u32::from_le_bytes([payload[at], payload[at + 1], payload[at + 2], payload[at + 3]]);
    if word(0) != 0 || word(12) != CONF_ASSOC {
        return None;
    }
    Some(match (word(4), word(8)) {
        (1, 1) => Session::Started,
        (1, _) => Session::Ended,
        _ => Session::Refused,
    })
}
