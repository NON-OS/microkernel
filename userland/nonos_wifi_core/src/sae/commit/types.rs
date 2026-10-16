// NONOS Operating System (AGPL-3.0-or-later)

//! One side's commit, and the keys it leads to.

use p256::{ProjectivePoint, Scalar};

/// One side's commit: the secret rand, the scalar and element it sends, and
/// the PWE they were built from.
#[derive(Clone)]
pub struct Commit {
    pub(super) rand: Scalar,
    pub(super) scalar: Scalar,
    pub(super) element: ProjectivePoint,
    pub(super) pwe: ProjectivePoint,
}

/// The keys a completed exchange yields.
#[derive(Clone)]
pub struct SaeKeys {
    pub kck: [u8; 32],
    pub pmk: [u8; 32],
    pub pmkid: [u8; 16],
}
