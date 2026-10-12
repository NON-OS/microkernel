//! The prover's refusals as the wallet's typed errors. Refusal text is
//! dropped, since it can name values and leaves.

use crate::error::ProveError;
use nox_prover::Error;

/// The prover's refusal as the wallet's typed error.
pub(crate) fn refusal(e: Error) -> ProveError {
    match e {
        Error::Cancelled => ProveError::Cancelled,
        Error::Entropy(_) => ProveError::SeedEntropy,
        Error::NotVerified(_) => ProveError::SelfVerify,
        Error::Request(why) if why.contains("does not balance") => ProveError::Unbalanced,
        Error::Request(why)
            if why.contains("rebuild to") || why.contains("is not the pool's leaf") =>
        {
            ProveError::RootNotPublished
        }
        _ => ProveError::WitnessUnsatisfied,
    }
}
