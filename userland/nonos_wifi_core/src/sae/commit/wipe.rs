// NONOS Operating System (AGPL-3.0-or-later)

//! A commit's secret rand and the PWE it was built from, and the keys an
//! exchange derived, wiped when they go.

use p256::{ProjectivePoint, Scalar};

use super::types::{Commit, SaeKeys};
use crate::wipe::{wipe, wipe_value};

impl Drop for Commit {
    fn drop(&mut self) {
        wipe_value(&mut self.rand, Scalar::ZERO);
        wipe_value(&mut self.scalar, Scalar::ZERO);
        wipe_value(&mut self.element, ProjectivePoint::IDENTITY);
        wipe_value(&mut self.pwe, ProjectivePoint::IDENTITY);
    }
}

impl Drop for SaeKeys {
    fn drop(&mut self) {
        wipe(&mut self.kck);
        wipe(&mut self.pmk);
        wipe(&mut self.pmkid);
    }
}
