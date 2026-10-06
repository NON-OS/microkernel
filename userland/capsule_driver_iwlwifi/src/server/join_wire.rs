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

//! The control family's join messages as this driver reads and answers them;
//! the client side is `nonos_wifi_client::join_wire`, and the RTL8821CE
//! driver answers the same bytes.
//!
//! A connect body is `[ssid_len][ssid][pass_len][pass]` then an optional
//! flags octet (bit 0: saved as WPA3, join with SAE or not at all; bit 1:
//! hidden). Every length is checked against the body, and an empty or
//! overlong SSID is refused. This driver hunts by listening only, so a hidden
//! network is not found.
//!
//! The connect reply is the status code, the exchange's counts (sent,
//! received, data, EAPOL, the first unanswered data frame's frame control
//! and length, deauthentications, frames to this station), the MLME state,
//! the AKM that ran and the access point's own status or reason code. Codes
//! -1 to -12 mean what they mean for the RTL8821CE (`join_text`); this
//! driver answers -1 too when the firmware failed one of the join's
//! commands. The link reply is `[associated][bssid][ssid_len][ssid][akm]`.

use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::SelectError;
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::wpa::supplicant::Failure;

use crate::firmware::gen3::join::exchange::Progress;
use crate::firmware::gen3::join::keys::KeyError;
use crate::firmware::gen3::join::run::JoinEnd;

const FLAG_WPA3_ONLY: u8 = 1 << 0;
const FLAG_HIDDEN: u8 = 1 << 1;
const SSID_MAX: usize = 32;
/// The control family header.
pub const WIFI_HDR: usize = 10;
/// The connect reply after the header.
pub const CONNECT_REPLY: usize = 36;

/// The radio is down, the request malformed, or the firmware failed a join
/// command.
pub const CODE_BAD_REQUEST: i32 = -1;
/// The network was not heard on any channel.
pub const CODE_NOT_FOUND: i32 = -2;
/// The pairwise or the group key did not go into the firmware.
pub const CODE_PAIRWISE_KEY: i32 = -3;
pub const CODE_GROUP_KEY: i32 = -4;
/// The access point refused authentication or association.
pub const CODE_REFUSED: i32 = -5;
/// The handshake did not finish (a wrong WPA2 passphrase ends here).
pub const CODE_TIMED_OUT: i32 = -6;
/// Saved as WPA3, the network now offers only WPA2: not joined.
pub const CODE_DOWNGRADE: i32 = -7;
/// Open, TKIP, Enterprise or 802.11n-only.
pub const CODE_UNSUPPORTED: i32 = -8;
/// Not 8 to 63 characters or 64 hex digits.
pub const CODE_BAD_PASSPHRASE: i32 = -9;
/// WPA3: the access point's SAE confirm did not verify.
pub const CODE_WRONG_PASSWORD: i32 = -10;
/// Message 3 did not carry the beacon's security elements.
pub const CODE_IE_MISMATCH: i32 = -11;
/// No randomness for the nonce and SAE secrets.
pub const CODE_NO_ENTROPY: i32 = -12;

/// A parsed connect request.
pub struct ConnectRequest<'a> {
    pub ssid: &'a [u8],
    pub pass: &'a [u8],
    pub wpa3_only: bool,
    pub hidden: bool,
}

/// Parse a connect body, or `None` if a length runs past it or the SSID is
/// empty or longer than 32 octets.
pub fn parse_connect(body: &[u8]) -> Option<ConnectRequest<'_>> {
    let ssid_len = *body.first()? as usize;
    if ssid_len == 0 || ssid_len > SSID_MAX {
        return None;
    }
    let ssid = body.get(1..1 + ssid_len)?;
    let pass_off = 1 + ssid_len;
    let pass_len = *body.get(pass_off)? as usize;
    let pass_end = pass_off + 1 + pass_len;
    let pass = body.get(pass_off + 1..pass_end)?;
    let flags = body.get(pass_end).copied().unwrap_or(0);
    Some(ConnectRequest { ssid, pass, wpa3_only: flags & FLAG_WPA3_ONLY != 0, hidden: flags & FLAG_HIDDEN != 0 })
}

/// A join's result for the reply.
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct ConnectResult {
    pub code: i32,
    pub progress: Progress,
    pub state: u8,
    pub akm: u8,
    pub ap_code: u16,
}

impl ConnectResult {
    pub fn code(code: i32) -> Self {
        Self { code, ..Default::default() }
    }
}

/// The status code, and the access point's own code, of a failed MLME.
pub fn failure_code(f: MlmeFailure) -> (i32, u16) {
    match f {
        MlmeFailure::Select(SelectError::Downgrade) => (CODE_DOWNGRADE, 0),
        MlmeFailure::Select(_) | MlmeFailure::OpenNetwork | MlmeFailure::MalformedRsne | MlmeFailure::NeedsHt => {
            (CODE_UNSUPPORTED, 0)
        }
        MlmeFailure::BadPassphrase => (CODE_BAD_PASSPHRASE, 0),
        MlmeFailure::NoEntropy => (CODE_NO_ENTROPY, 0),
        MlmeFailure::Sae(SaeFailure::BadConfirm) => (CODE_WRONG_PASSWORD, 0),
        MlmeFailure::Sae(SaeFailure::Rejected(status)) => (CODE_REFUSED, status),
        MlmeFailure::Sae(_) => (CODE_REFUSED, 0),
        MlmeFailure::AuthRejected(status) | MlmeFailure::AssocRejected(status) => (CODE_REFUSED, status),
        MlmeFailure::Left(reason) => (CODE_TIMED_OUT, reason),
        MlmeFailure::Handshake(Failure::IeMismatch) => (CODE_IE_MISMATCH, 0),
        MlmeFailure::Handshake(_) => (CODE_TIMED_OUT, 0),
    }
}

/// The status code, and the access point's own code, of a join's end.
pub fn end_code(e: JoinEnd) -> (i32, u16) {
    match e {
        JoinEnd::NotTheNetwork => (CODE_NOT_FOUND, 0),
        JoinEnd::Refused(f) => failure_code(f),
        JoinEnd::TimedOut => (CODE_TIMED_OUT, 0),
        JoinEnd::Setup(_) | JoinEnd::Firmware(_) => (CODE_BAD_REQUEST, 0),
        JoinEnd::Keys(KeyError::Pairwise(_)) => (CODE_PAIRWISE_KEY, 0),
        JoinEnd::Keys(KeyError::Group(_)) => (CODE_GROUP_KEY, 0),
    }
}

/// Write the connect reply after the header in `out`; its length with the
/// header, or `None` when `out` is too small.
pub fn encode_connect(out: &mut [u8], r: &ConnectResult) -> Option<usize> {
    let b = out.get_mut(WIFI_HDR..WIFI_HDR + CONNECT_REPLY)?;
    let p = &r.progress;
    let words = [r.code as u32, p.sent, p.recv, p.data, p.eapol, p.probe, p.deauth, p.to_us];
    for (i, w) in words.iter().enumerate() {
        b[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
    }
    b[32] = r.state;
    b[33] = r.akm;
    b[34..36].copy_from_slice(&r.ap_code.to_le_bytes());
    Some(WIFI_HDR + CONNECT_REPLY)
}

/// The association a link reply describes.
pub struct LinkInfo<'a> {
    pub bssid: [u8; 6],
    pub ssid: &'a [u8],
    /// The AKM suite type that ran (2 WPA2-PSK, 6 PSK-SHA256, 8 SAE).
    pub akm: u8,
}

/// Write the link reply after the header: not associated, or `link`.
pub fn encode_link(out: &mut [u8], link: Option<&LinkInfo<'_>>) -> Option<usize> {
    let fixed = WIFI_HDR + 8;
    out.get_mut(WIFI_HDR..fixed)?.fill(0);
    let Some(l) = link else { return Some(fixed) };
    let len = l.ssid.len().min(SSID_MAX);
    out[WIFI_HDR] = 1;
    out[WIFI_HDR + 1..WIFI_HDR + 7].copy_from_slice(&l.bssid);
    out[fixed - 1] = len as u8;
    out.get_mut(fixed..fixed + len)?.copy_from_slice(&l.ssid[..len]);
    *out.get_mut(fixed + len)? = l.akm;
    Some(fixed + len + 1)
}

/// Write a status-only reply after the header.
pub fn encode_code(out: &mut [u8], code: i32) -> Option<usize> {
    out.get_mut(WIFI_HDR..WIFI_HDR + 4)?.copy_from_slice(&code.to_le_bytes());
    Some(WIFI_HDR + 4)
}
