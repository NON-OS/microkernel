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

//! What a connect reports, and the status code each ending maps to. Codes
//! -1 to -6 keep the meanings `nonos_wifi_client` already shows; the rest
//! name the WPA3 and security outcomes a person can act on.

use crate::mlme::MlmeFailure;
use crate::rsn::SelectError;
use crate::sae::SaeFailure;
use crate::wpa::supplicant::Failure;

pub const CODE_BAD_REQUEST: i32 = -1;
pub const CODE_NOT_FOUND: i32 = -2;
pub const CODE_REFUSED: i32 = -5;
pub const CODE_TIMED_OUT: i32 = -6;
pub const CODE_DOWNGRADE: i32 = -7;
pub const CODE_UNSUPPORTED: i32 = -8;
pub const CODE_BAD_PASSPHRASE: i32 = -9;
pub const CODE_WRONG_PASSWORD: i32 = -10;
pub const CODE_IE_MISMATCH: i32 = -11;
pub const CODE_NO_ENTROPY: i32 = -12;
/// SAE could not start from the join's randomness; the drivers' code too.
pub const CODE_SAE_START: i32 = -14;
/// The radio could not take the network's address (`crate::address`).
pub const CODE_NO_ADDRESS: i32 = -15;

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
        MlmeFailure::SaeStart => (CODE_SAE_START, 0),
        MlmeFailure::Sae(SaeFailure::BadConfirm) => (CODE_WRONG_PASSWORD, 0),
        MlmeFailure::Sae(SaeFailure::Rejected(status)) => (CODE_REFUSED, status),
        MlmeFailure::Sae(_) => (CODE_REFUSED, 0),
        MlmeFailure::AuthRejected(s) | MlmeFailure::AssocRejected(s) => (CODE_REFUSED, s),
        MlmeFailure::Left(reason) => (CODE_TIMED_OUT, reason),
        MlmeFailure::Handshake(Failure::IeMismatch) => (CODE_IE_MISMATCH, 0),
        MlmeFailure::Handshake(_) => (CODE_TIMED_OUT, 0),
        // A step the access point never answered within its own time.
        MlmeFailure::Timeout(_) => (CODE_TIMED_OUT, 0),
    }
}
