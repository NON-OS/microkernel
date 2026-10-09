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

//! What a join reports back, and the status code each way it can end maps to.
//! Codes -1 to -6 keep the meanings the client already shows; the rest name
//! the WPA3 and security outcomes a person can act on (the network now offers
//! only WPA2 though it was saved as WPA3, the passphrase is not one, the SAE
//! confirm failed, message 3 did not match the beacon).

use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::SelectError;
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::wpa::supplicant::Failure;

/// The radio is down or the request was malformed.
pub const CODE_BAD_REQUEST: i32 = -1;
/// The network was not heard on any channel.
pub const CODE_NOT_FOUND: i32 = -2;
/// The access point refused authentication or association.
pub const CODE_REFUSED: i32 = -5;
/// The handshake did not finish (a WPA2 passphrase mismatch ends here).
pub const CODE_TIMED_OUT: i32 = -6;
/// The network was saved as WPA3 and now offers only WPA2: not joined.
pub const CODE_DOWNGRADE: i32 = -7;
/// The network's security is not one this station runs (open, TKIP,
/// Enterprise, 802.11n-only).
pub const CODE_UNSUPPORTED: i32 = -8;
/// The passphrase is not 8 to 63 characters or 64 hex digits.
pub const CODE_BAD_PASSPHRASE: i32 = -9;
/// WPA3: the access point's SAE confirm did not verify (a wrong password).
pub const CODE_WRONG_PASSWORD: i32 = -10;
/// Message 3 did not carry the beacon's security elements: possibly an
/// attempted downgrade; the join was abandoned.
pub const CODE_IE_MISMATCH: i32 = -11;
/// No randomness for the station's nonces and SAE secrets.
pub const CODE_NO_ENTROPY: i32 = -12;

/// A connection attempt's result: the status code, and the handshake progress
/// (frames sent and received, and the state reached) for the panel to show,
/// then the AKM that ran (0 none, 2 WPA2-PSK, 6 PSK-SHA256, 8 WPA3-SAE) and
/// the access point's own status or reason code, when it gave one.
#[derive(Default)]
pub struct ConnectResult {
    pub code: i32,
    pub sent: u32,
    pub recv: u32,
    pub data: u32,
    pub eapol: u32,
    pub probe: u32,
    pub deauth: u32,
    pub to_us: u32,
    pub state: u8,
    pub akm: u8,
    pub ap_code: u16,
}

impl ConnectResult {
    /// A result with only a status code.
    pub fn code(code: i32) -> Self {
        Self { code, ..Default::default() }
    }
}

/// The status code and the AP's own code for a join that failed with `f`.
pub fn failure_code(f: MlmeFailure) -> (i32, u16) {
    match f {
        MlmeFailure::Select(SelectError::Downgrade) => (CODE_DOWNGRADE, 0),
        MlmeFailure::Select(_)
        | MlmeFailure::OpenNetwork
        | MlmeFailure::MalformedRsne
        | MlmeFailure::NeedsHt => (CODE_UNSUPPORTED, 0),
        MlmeFailure::BadPassphrase => (CODE_BAD_PASSPHRASE, 0),
        MlmeFailure::NoEntropy => (CODE_NO_ENTROPY, 0),
        MlmeFailure::Sae(SaeFailure::BadConfirm) => (CODE_WRONG_PASSWORD, 0),
        MlmeFailure::Sae(SaeFailure::Rejected(status)) => (CODE_REFUSED, status),
        MlmeFailure::Sae(_) => (CODE_REFUSED, 0),
        MlmeFailure::AuthRejected(status) | MlmeFailure::AssocRejected(status) => {
            (CODE_REFUSED, status)
        }
        MlmeFailure::Left(reason) => (CODE_TIMED_OUT, reason),
        MlmeFailure::Handshake(Failure::IeMismatch) => (CODE_IE_MISMATCH, 0),
        MlmeFailure::Handshake(_) => (CODE_TIMED_OUT, 0),
    }
}
