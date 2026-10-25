// NONOS Operating System (AGPL-3.0-or-later)
//! Every way a join ends maps to the code the panels name.

use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::SelectError;
use nonos_wifi_core::sae::SaeFailure;
use nonos_wifi_core::wpa::supplicant::Failure;

use crate::gen3::dev::WaitError;
use crate::gen3::join::bss::SetupError;
use crate::gen3::join::fw::CommandFailed;
use crate::gen3::join::keys::KeyError;
use crate::gen3::join::run::JoinEnd;
use crate::join_wire::*;

#[test]
fn every_end_maps_to_the_code_the_panels_name() {
    let fail = CommandFailed { group: 1, cmd: 8, why: WaitError::TimedOut };
    let cases = [
        (JoinEnd::NotTheNetwork, (CODE_NOT_FOUND, 0)),
        (JoinEnd::TimedOut, (CODE_TIMED_OUT, 0)),
        (JoinEnd::Setup(SetupError::BadQueueReply), (CODE_BAD_REQUEST, 0)),
        (JoinEnd::Firmware(Some(fail)), (CODE_BAD_REQUEST, 0)),
        (JoinEnd::Firmware(None), (CODE_BAD_REQUEST, 0)),
        (JoinEnd::Keys(KeyError::Pairwise(fail)), (CODE_PAIRWISE_KEY, 0)),
        (JoinEnd::Keys(KeyError::Group(None)), (CODE_GROUP_KEY, 0)),
        (JoinEnd::Refused(MlmeFailure::AuthRejected(17)), (CODE_REFUSED, 17)),
        (JoinEnd::Refused(MlmeFailure::AssocRejected(18)), (CODE_REFUSED, 18)),
        (JoinEnd::Refused(MlmeFailure::Left(15)), (CODE_TIMED_OUT, 15)),
        (JoinEnd::Refused(MlmeFailure::Select(SelectError::Downgrade)), (CODE_DOWNGRADE, 0)),
        (JoinEnd::Refused(MlmeFailure::OpenNetwork), (CODE_UNSUPPORTED, 0)),
        (JoinEnd::Refused(MlmeFailure::NeedsHt), (CODE_UNSUPPORTED, 0)),
        (JoinEnd::Refused(MlmeFailure::BadPassphrase), (CODE_BAD_PASSPHRASE, 0)),
        (JoinEnd::Refused(MlmeFailure::SaeStart), (CODE_SAE_START, 0)),
        (JoinEnd::Refused(MlmeFailure::Sae(SaeFailure::BadConfirm)), (CODE_WRONG_PASSWORD, 0)),
        (JoinEnd::Refused(MlmeFailure::Sae(SaeFailure::Rejected(77))), (CODE_REFUSED, 77)),
        (JoinEnd::Refused(MlmeFailure::Handshake(Failure::IeMismatch)), (CODE_IE_MISMATCH, 0)),
    ];
    for (end, want) in cases {
        assert_eq!(end_code(end), want, "{end:?}");
    }
}
