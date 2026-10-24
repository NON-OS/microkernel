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

//! The post-quantum key generation under the quantum key vault, which is
//! not the point here: a keypair made from a count, so each is known and no
//! two are alike.

use std::sync::atomic::{AtomicU8, Ordering};

static PAIRS: AtomicU8 = AtomicU8::new(1);

pub mod pq_ops {
    use crate::security::quantum::pqc::types::QuantumAlgorithm;

    /// The `n`th pair: a public key of 32 bytes `n`, its secret their inverse.
    pub fn generate_pq_keypair(
        _algo: &QuantumAlgorithm,
    ) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
        let n = super::PAIRS.fetch_add(1, super::Ordering::Relaxed);
        Ok((vec![n; 32], vec![!n; 32]))
    }
}
