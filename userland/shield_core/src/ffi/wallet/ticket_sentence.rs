//! The pool's refusal of a deposit, in the sentence a screen shows.

use crate::wallet::Refusal;

/// The pool's refusal, the way a screen says it.
pub(in crate::ffi::wallet) fn sentence(refusal: Refusal) -> &'static str {
    match refusal {
        Refusal::DepositsPaused => "The pool has paused deposits.",
        Refusal::WoundDown => "This pool is wound down. Money sent to it could not be spent.",
        Refusal::PolicyPaused => {
            "The pool's policy is paused. Money sent now could not leave until it resumes."
        }
        Refusal::Amount => "That amount is zero, too large for one note, or all fee.",
        Refusal::NotWholeUnits => "The pool takes whole units only. Round the amount.",
        Refusal::BetaPaused => "The beta is paused.",
        Refusal::NotOnBetaList => "That EVM address is not on the beta list.",
        Refusal::AddressCapReached => "That EVM address has reached its beta limit.",
        Refusal::PoolCapReached => "The pool has reached its beta limit.",
    }
}
