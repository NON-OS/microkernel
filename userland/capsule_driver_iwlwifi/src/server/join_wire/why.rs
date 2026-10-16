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

//! The status code, and the access point's own code, for each way a join
//! ends without a link.

use super::codes::{
    CODE_BAD_PASSPHRASE, CODE_BAD_REQUEST, CODE_DOWNGRADE, CODE_GROUP_KEY, CODE_IE_MISMATCH,
    CODE_NOT_FOUND, CODE_PAIRWISE_KEY, CODE_REFUSED, CODE_SAE_START, CODE_TIMED_OUT,
    CODE_UNSUPPORTED, CODE_WRONG_PASSWORD,
};
use crate::firmware::gen3::join::keys::KeyError;
use crate::firmware::gen3::join::run::JoinEnd;
use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::SelectError;
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::wpa::supplicant::Failure;

/// The status code, and the access point's own code, of a failed MLME.
pub fn failure_code(f: MlmeFailure) -> (i32, u16) {
    match f {
        MlmeFailure::Select(SelectError::Downgrade) => (CODE_DOWNGRADE, 0),
        MlmeFailure::Select(_) | MlmeFailure::OpenNetwork | MlmeFailure::MalformedRsne | MlmeFailure::NeedsHt => {
            (CODE_UNSUPPORTED, 0)
        }
        MlmeFailure::BadPassphrase => (CODE_BAD_PASSPHRASE, 0),
        MlmeFailure::SaeStart => (CODE_SAE_START, 0),
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
