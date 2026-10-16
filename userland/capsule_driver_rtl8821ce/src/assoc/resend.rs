// NONOS Operating System (AGPL-3.0-or-later)

//! When an unanswered frame goes again.

use super::send::FC0_AUTH;
use nonos_wifi_core::dot11::header::MAC_HEADER_LEN;
use nonos_wifi_core::sae::frame::AUTH_ALG_SAE;

/// Milliseconds the last transmitted frame may go unanswered before it is resent.
/// A Wi-Fi link loses frames and the state machine sends each once, so a lost
/// authentication or association frame is retransmitted this often until the
/// peer replies. Counted on the uptime clock: a count of receive passes ran at
/// whatever rate the passes did, which changed when every core started running.
/// mac80211 waits the same for an authentication or association answer
/// (`IEEE80211_AUTH_TIMEOUT`, `IEEE80211_ASSOC_TIMEOUT`).
pub const RETX_AFTER_MS: u64 = 200;

/// Milliseconds an SAE authentication frame may go unanswered before it is
/// resent. The router computes its commit before it answers, and every commit
/// it takes while committed counts toward the limit (`dot11RSNASAESync`, 5 in
/// hostapd) after which it abandons the exchange, so a commit is resent no
/// sooner than mac80211 resends one (`IEEE80211_AUTH_TIMEOUT_SAE`).
pub const RETX_SAE_AFTER_MS: u64 = 2_000;

// How long `mpdu` may go unanswered before it is resent: an SAE authentication
// frame waits for the router's computation, anything else the usual interval.
pub(super) fn resend_after(mpdu: &[u8]) -> u64 {
    let sae = mpdu.len() >= MAC_HEADER_LEN + 2
        && mpdu[0] == FC0_AUTH
        && u16::from_le_bytes([mpdu[MAC_HEADER_LEN], mpdu[MAC_HEADER_LEN + 1]]) == AUTH_ALG_SAE;
    if sae {
        RETX_SAE_AFTER_MS
    } else {
        RETX_AFTER_MS
    }
}
