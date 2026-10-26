// NONOS Operating System (AGPL-3.0-or-later)

//! Whether a failed join goes on to the next candidate.

use crate::mlme::MlmeFailure;

/// Whether a join that failed with `failure` (`None`: the access point went
/// quiet) was ended by the access point: it refused, went quiet (here, or
/// within a step's own deadline), left, or
/// could not prove it holds the password, as an impostor cannot. Another
/// access point of the network may still be joined. A failure on this side
/// (a passphrase that cannot be used, SAE that could not start) would fail
/// the same way on every candidate, and ends the request.
pub fn moves_on(failure: Option<MlmeFailure>) -> bool {
    use MlmeFailure::*;
    match failure {
        None => true,
        Some(f) => {
            matches!(
                f,
                AuthRejected(_) | AssocRejected(_) | Sae(_) | Left(_) | Handshake(_) | Timeout(_)
            )
        }
    }
}
