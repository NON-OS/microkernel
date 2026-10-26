// NONOS Operating System (AGPL-3.0-or-later)

//! Each way a join ends has its code, with the meanings the client shows.

use nonos_wifi_core::mlme::{MlmeFailure, MlmeState};
use nonos_wifi_core::rsn::SelectError;
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::wpa::supplicant::Failure;

use crate::connect_result::*;

#[test]
fn each_way_a_join_ends_has_its_own_code() {
    let cases = [
        (MlmeFailure::Select(SelectError::Downgrade), (CODE_DOWNGRADE, 0)),
        (MlmeFailure::Select(SelectError::UnsupportedAkm), (CODE_UNSUPPORTED, 0)),
        (MlmeFailure::Select(SelectError::UnsupportedCipher), (CODE_UNSUPPORTED, 0)),
        (MlmeFailure::OpenNetwork, (CODE_UNSUPPORTED, 0)),
        (MlmeFailure::MalformedRsne, (CODE_UNSUPPORTED, 0)),
        (MlmeFailure::NeedsHt, (CODE_UNSUPPORTED, 0)),
        (MlmeFailure::BadPassphrase, (CODE_BAD_PASSPHRASE, 0)),
        (MlmeFailure::SaeStart, (CODE_SAE_START, 0)),
        (MlmeFailure::Sae(SaeFailure::BadConfirm), (CODE_WRONG_PASSWORD, 0)),
        (MlmeFailure::Sae(SaeFailure::Rejected(77)), (CODE_REFUSED, 77)),
        (MlmeFailure::Sae(SaeFailure::GroupNotSupported), (CODE_REFUSED, 0)),
        (MlmeFailure::AuthRejected(1), (CODE_REFUSED, 1)),
        (MlmeFailure::AssocRejected(17), (CODE_REFUSED, 17)),
        (MlmeFailure::Left(15), (CODE_TIMED_OUT, 15)),
        (MlmeFailure::Handshake(Failure::IeMismatch), (CODE_IE_MISMATCH, 0)),
        (MlmeFailure::Handshake(Failure::BadKeyData), (CODE_TIMED_OUT, 0)),
        (MlmeFailure::Timeout(MlmeState::Associating), (CODE_TIMED_OUT, 0)),
    ];
    for (f, want) in cases {
        assert_eq!(failure_code(f), want);
    }
}

#[test]
fn the_codes_keep_the_meanings_the_client_already_shows() {
    assert_eq!(
        [CODE_BAD_REQUEST, CODE_NOT_FOUND, CODE_REFUSED, CODE_TIMED_OUT],
        [-1, -2, -5, -6],
        "the existing codes are unchanged"
    );
    let new = [
        CODE_DOWNGRADE,
        CODE_UNSUPPORTED,
        CODE_BAD_PASSPHRASE,
        CODE_WRONG_PASSWORD,
        CODE_IE_MISMATCH,
        CODE_NO_ENTROPY,
    ];
    assert_eq!(new, [-7, -8, -9, -10, -11, -12], "the new ones follow them");
    let r = ConnectResult::code(CODE_NOT_FOUND);
    assert_eq!((r.code, r.akm, r.ap_code, r.sent), (CODE_NOT_FOUND, 0, 0, 0));
}
