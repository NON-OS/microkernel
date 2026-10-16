// NONOS Operating System (AGPL-3.0-or-later)

//! Which code each way a join can end maps to.

use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::SelectError;
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::wpa::supplicant::Failure;

use super::codes::{
    CODE_BAD_PASSPHRASE, CODE_DOWNGRADE, CODE_IE_MISMATCH, CODE_NO_ENTROPY, CODE_REFUSED,
    CODE_TIMED_OUT, CODE_UNSUPPORTED, CODE_WRONG_PASSWORD,
};

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
